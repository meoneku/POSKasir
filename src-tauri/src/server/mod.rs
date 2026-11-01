use crate::db::Database;
use crate::models::{CreateTransactionInput, Product};
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State as AxumState,
    },
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use chrono::Local;
use futures_util::{SinkExt, StreamExt};
use local_ip_address::local_ip;
use parking_lot::Mutex as SyncMutex;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::{broadcast, oneshot, Mutex as AsyncMutex};
use tower_http::cors::CorsLayer;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub is_running: bool,
    pub host_ip: String,
    pub port: u16,
    pub connected_clients: usize,
    pub store_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WsEvent {
    pub event_type: String, // 'STOCK_UPDATED', 'TRANSACTION_CREATED', 'PING'
    pub payload: serde_json::Value,
    pub timestamp: String,
}

#[derive(Clone)]
pub struct AppServerState {
    pub db: Database,
    pub ws_broadcast_tx: broadcast::Sender<String>,
    pub connected_clients: Arc<AtomicUsize>,
}

#[derive(Clone)]
pub struct ServerManager {
    pub is_running: Arc<AtomicBool>,
    pub port: Arc<SyncMutex<u16>>,
    pub host_ip: Arc<SyncMutex<String>>,
    pub shutdown_sender: Arc<AsyncMutex<Option<oneshot::Sender<()>>>>,
    pub broadcast_tx: broadcast::Sender<String>,
    pub connected_clients: Arc<AtomicUsize>,
    pub db: Database,
}

impl ServerManager {
    pub fn new(db: Database) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);
        let default_ip = local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            port: Arc::new(SyncMutex::new(8080)),
            host_ip: Arc::new(SyncMutex::new(default_ip)),
            shutdown_sender: Arc::new(AsyncMutex::new(None)),
            broadcast_tx,
            connected_clients: Arc::new(AtomicUsize::new(0)),
            db,
        }
    }

    pub fn get_info(&self) -> ServerInfo {
        let is_running = self.is_running.load(Ordering::SeqCst);
        let port = *self.port.lock();
        let host_ip = self.host_ip.lock().clone();
        let connected_clients = self.connected_clients.load(Ordering::SeqCst);

        // Ambil nama toko dari database
        let mut store_name = "BERKAH POS KASIR".to_string();
        if let Ok(mut stmt) = self
            .db
            .conn
            .lock()
            .prepare("SELECT setting_value FROM app_settings WHERE setting_key = 'store_name'")
        {
            if let Ok(mut rows) = stmt.query([]) {
                if let Ok(Some(row)) = rows.next() {
                    store_name = row.get(0).unwrap_or(store_name);
                }
            }
        }

        ServerInfo {
            is_running,
            host_ip,
            port,
            connected_clients,
            store_name,
        }
    }

    pub async fn start(&self, custom_port: Option<u16>) -> Result<ServerInfo, String> {
        if self.is_running.load(Ordering::SeqCst) {
            return Ok(self.get_info());
        }

        let port = custom_port.unwrap_or(8080);
        *self.port.lock() = port;

        // Ambil IP Lokal terkini
        let current_ip = local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        *self.host_ip.lock() = current_ip;

        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| format!("Gagal bind ke port {}: {}", port, e))?;

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        *self.shutdown_sender.lock().await = Some(shutdown_tx);

        let app_state = AppServerState {
            db: self.db.clone(),
            ws_broadcast_tx: self.broadcast_tx.clone(),
            connected_clients: self.connected_clients.clone(),
        };

        let app = Router::new()
            .route("/api/v1/ping", get(handle_ping))
            .route("/api/v1/catalog", get(handle_get_catalog))
            .route("/api/v1/transactions", post(handle_create_transaction))
            .route("/api/v1/reports/summary", get(handle_get_summary))
            .route("/api/v1/sync/status", get(handle_sync_status))
            .route("/api/v1/sync/push", post(handle_sync_push))
            .route("/api/v1/sync/pull", get(handle_sync_pull))
            .route("/api/v1/sync/branch-stocks", get(handle_branch_stocks))
            .route("/ws", get(handle_ws_upgrade))
            .layer(CorsLayer::permissive())
            .with_state(app_state);

        let is_running_clone = self.is_running.clone();
        is_running_clone.store(true, Ordering::SeqCst);

        // Spawn web server di background tokio task
        tokio::spawn(async move {
            println!("Embedded Kasir Server aktif di http://{}", addr);
            if let Err(err) = axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                    println!("Embedded Kasir Server menerima sinyal shutdown.");
                })
                .await
            {
                eprintln!("Error pada embedded server: {}", err);
            }
            is_running_clone.store(false, Ordering::SeqCst);
        });

        Ok(self.get_info())
    }

    pub async fn stop(&self) -> Result<(), String> {
        let mut sender_lock = self.shutdown_sender.lock().await;
        if let Some(sender) = sender_lock.take() {
            let _ = sender.send(());
            self.is_running.store(false, Ordering::SeqCst);
            println!("Server lokal dihentikan.");
        }
        Ok(())
    }

    pub fn broadcast(&self, event_type: &str, payload: serde_json::Value) {
        let event = WsEvent {
            event_type: event_type.to_string(),
            payload,
            timestamp: Local::now().to_rfc3339(),
        };
        if let Ok(json) = serde_json::to_string(&event) {
            let _ = self.broadcast_tx.send(json);
        }
    }
}

// --- HANDLERS REST API ---

async fn handle_ping(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let store_name = {
        let conn = state.db.conn.lock();
        conn.query_row(
            "SELECT setting_value FROM app_settings WHERE setting_key = 'store_name'",
            [],
            |r| r.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "BERKAH POS KASIR".to_string())
    };

    Json(serde_json::json!({
        "status": "online",
        "store_name": store_name,
        "role": "master_server",
        "timestamp": Local::now().to_rfc3339(),
        "connected_clients": state.connected_clients.load(Ordering::SeqCst)
    }))
}

async fn handle_get_catalog(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let conn = state.db.conn.lock();

    // 1. Ambil produk aktif
    let mut prod_stmt = match conn.prepare(
        "SELECT p.id, p.sku, p.barcode, p.name, p.category_id, c.name as category_name,
                p.min_stock_alert, p.is_active, p.is_taxable, p.tax_id, p.base_unit_name, p.current_stock
         FROM products p
         LEFT JOIN categories c ON p.category_id = c.id
         WHERE p.is_active = 1
         ORDER BY p.name ASC",
    ) {
        Ok(stmt) => stmt,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    };

    let prod_rows = match prod_stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            sku: row.get(1)?,
            barcode: row.get(2)?,
            name: row.get(3)?,
            category_id: row.get(4)?,
            category_name: row.get(5)?,
            min_stock_alert: row.get(6)?,
            is_active: row.get(7)?,
            is_taxable: row.get(8)?,
            tax_id: row.get(9)?,
            base_unit_name: row.get(10)?,
            current_stock: row.get(11)?,
            units: Vec::new(),
        })
    }) {
        Ok(rows) => rows,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    };

    let mut products = Vec::new();
    for p in prod_rows {
        if let Ok(mut prod) = p {
            if let Ok(mut unit_stmt) = conn.prepare(
                "SELECT id, product_id, unit_name, multiplier, barcode, retail_price,
                        wholesale_price_1, wholesale_price_2, min_qty_wholesale_1, min_qty_wholesale_2
                 FROM product_units WHERE product_id = ?1 ORDER BY multiplier ASC",
            ) {
                if let Ok(unit_rows) = unit_stmt.query_map([&prod.id], |r| {
                    Ok(crate::models::ProductUnit {
                        id: r.get(0)?,
                        product_id: r.get(1)?,
                        unit_name: r.get(2)?,
                        multiplier: r.get(3)?,
                        barcode: r.get(4)?,
                        retail_price: r.get(5)?,
                        wholesale_price_1: r.get(6)?,
                        wholesale_price_2: r.get(7)?,
                        min_qty_wholesale_1: r.get(8)?,
                        min_qty_wholesale_2: r.get(9)?,
                    })
                }) {
                    for u in unit_rows {
                        if let Ok(unit) = u {
                            prod.units.push(unit);
                        }
                    }
                }
            }
            products.push(prod);
        }
    }

    (StatusCode::OK, Json(serde_json::json!({ "products": products })))
}

async fn handle_create_transaction(
    AxumState(state): AxumState<AppServerState>,
    Json(input): Json<CreateTransactionInput>,
) -> impl IntoResponse {
    let mut conn = state.db.conn.lock();
    let tx = match conn.transaction() {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
        }
    };

    let tx_id = uuid::Uuid::new_v4().to_string();
    let date_str = Local::now().format("%Y%m%d").to_string();
    let rand_seq = Local::now().format("%H%M%S").to_string();
    let invoice_num = format!("INV/{}/{}", date_str, rand_seq);

    // 1. Insert Transaction Header
    if let Err(e) = tx.execute(
        "INSERT INTO transactions (id, invoice_number, order_type, shift_id, customer_id, cashier_id,
                                   subtotal, discount_total, tax_mode, tax_base_amount, tax_total, grand_total, payment_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 'paid')",
        rusqlite::params![
            tx_id,
            invoice_num,
            input.order_type,
            input.shift_id,
            input.customer_id,
            input.cashier_id,
            input.subtotal,
            input.discount_total,
            input.tax_mode,
            input.tax_base_amount,
            input.tax_total,
            input.grand_total,
        ],
    ) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        );
    }

    let mut stock_updates = Vec::new();

    // 2. Insert Transaction Items & Update Stock Atomic
    for item in &input.items {
        let item_id = uuid::Uuid::new_v4().to_string();
        let base_qty = item.quantity * item.multiplier;

        if let Err(e) = tx.execute(
            "INSERT INTO transaction_items (id, transaction_id, product_id, unit_id, unit_name, multiplier,
                                           quantity, base_qty, unit_price, discount, is_taxable, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                item_id,
                tx_id,
                item.product_id,
                item.unit_id,
                item.unit_name,
                item.multiplier,
                item.quantity,
                base_qty,
                item.unit_price,
                item.discount,
                item.is_taxable,
                item.subtotal,
            ],
        ) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            );
        }

        // Deduct Stock
        let _ = tx.execute(
            "UPDATE products SET current_stock = current_stock - ?1 WHERE id = ?2",
            rusqlite::params![base_qty, item.product_id],
        );

        let mutation_id = uuid::Uuid::new_v4().to_string();
        let mutation_type = if input.order_type == "wholesale" {
            "sale_wholesale"
        } else {
            "sale_retail"
        };

        let _ = tx.execute(
            "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_out, ending_stock, note)
             SELECT ?1, ?2, ?3, ?4, ?5, current_stock, 'Penjualan Klien LAN' FROM products WHERE id = ?2",
            rusqlite::params![mutation_id, item.product_id, mutation_type, invoice_num, base_qty],
        );

        // Ambil stok baru untuk broadcast
        let new_stock: i64 = tx
            .query_row(
                "SELECT current_stock FROM products WHERE id = ?1",
                [&item.product_id],
                |r| r.get(0),
            )
            .unwrap_or(0);

        stock_updates.push(serde_json::json!({
            "product_id": item.product_id,
            "new_stock": new_stock
        }));
    }

    // 3. Insert Payments
    for p in &input.payments {
        let pay_id = uuid::Uuid::new_v4().to_string();
        let _ = tx.execute(
            "INSERT INTO payments (id, transaction_id, payment_method_id, amount_paid, change_due, reference_number)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                pay_id,
                tx_id,
                p.payment_method_id,
                p.amount_paid,
                input.change_due,
                p.reference_number,
            ],
        );
    }

    if let Err(e) = tx.commit() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        );
    }

    // 4. Broadcast Realtime Event ke seluruh klien kasir yang terhubung
    let event = WsEvent {
        event_type: "STOCK_UPDATED".to_string(),
        payload: serde_json::json!({
            "invoice_number": invoice_num,
            "stock_updates": stock_updates,
        }),
        timestamp: Local::now().to_rfc3339(),
    };
    if let Ok(json_str) = serde_json::to_string(&event) {
        let _ = state.ws_broadcast_tx.send(json_str);
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "invoice_number": invoice_num,
            "transaction_id": tx_id
        })),
    )
}

async fn handle_get_summary(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let conn = state.db.conn.lock();
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut today_sales = 0.0;
    let mut today_count = 0;

    let _ = conn.query_row(
        "SELECT COALESCE(SUM(grand_total), 0.0), COUNT(id) FROM transactions WHERE date(created_at) = date(?1)",
        [&today],
        |r| {
            today_sales = r.get(0).unwrap_or(0.0);
            today_count = r.get(1).unwrap_or(0);
            Ok(())
        },
    );

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "today_sales": today_sales,
            "today_transactions_count": today_count
        })),
    )
}

// --- WEBSOCKET HANDLER ---

async fn handle_ws_upgrade(
    ws: WebSocketUpgrade,
    AxumState(state): AxumState<AppServerState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(socket: WebSocket, state: AppServerState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.ws_broadcast_tx.subscribe();

    state.connected_clients.fetch_add(1, Ordering::SeqCst);
    println!(
        "Klien kasir baru terhubung via WebSocket. Total klien: {}",
        state.connected_clients.load(Ordering::SeqCst)
    );

    // Kirim pesan selamat datang
    let welcome = serde_json::json!({
        "event_type": "CONNECTED",
        "message": "Terhubung ke Server Kasir Utama",
        "timestamp": Local::now().to_rfc3339()
    });
    if let Ok(msg) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(msg)).await;
    }

    // Task untuk forward broadcast dari server ke websocket client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Task untuk menerima pesan/ping dari client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // Tunggu sampai salah satu task selesai (klien diskonek)
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    state.connected_clients.fetch_sub(1, Ordering::SeqCst);
    println!(
        "Klien kasir terputus dari WebSocket. Sisa klien: {}",
        state.connected_clients.load(Ordering::SeqCst)
    );
}

// --- SPRINT 6: MULTI-CABANG SYNC ENDPOINTS ---

async fn handle_sync_status(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let conn = state.db.conn.lock();
    let tx_count: i64 = conn.query_row("SELECT COUNT(*) FROM transactions", [], |r| r.get(0)).unwrap_or(0);
    let prod_count: i64 = conn.query_row("SELECT COUNT(*) FROM products", [], |r| r.get(0)).unwrap_or(0);
    let branch_count: i64 = conn.query_row("SELECT COUNT(*) FROM branches", [], |r| r.get(0)).unwrap_or(0);

    Json(serde_json::json!({
        "status": "online",
        "service": "central_sync_hub",
        "total_branches": branch_count,
        "total_products": prod_count,
        "total_transactions": tx_count,
        "timestamp": Local::now().to_rfc3339()
    }))
}

async fn handle_sync_push(
    AxumState(state): AxumState<AppServerState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let branch_code = payload.get("branch_code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
    let tx_items = payload.get("transactions").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    let trf_items = payload.get("transfers").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);

    let broadcast_event = serde_json::json!({
        "branch_code": branch_code,
        "pushed_transactions": tx_items,
        "pushed_transfers": trf_items,
        "synced_at": Local::now().to_rfc3339()
    });
    let _ = state.ws_broadcast_tx.send(serde_json::json!({
        "event_type": "BRANCH_SYNC_RECEIVED",
        "payload": broadcast_event,
        "timestamp": Local::now().to_rfc3339()
    }).to_string());

    (StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "message": format!("Menerima {} transaksi & {} mutasi dari cabang {}", tx_items, trf_items, branch_code),
        "received_at": Local::now().to_rfc3339()
    })))
}

async fn handle_sync_pull(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let conn = state.db.conn.lock();
    let mut stmt = match conn.prepare("SELECT id, sku, barcode, name, current_stock FROM products WHERE is_active = 1") {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))),
    };

    let items = stmt.query_map([], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, String>(0)?,
            "sku": r.get::<_, String>(1)?,
            "barcode": r.get::<_, Option<String>>(2)?,
            "name": r.get::<_, String>(3)?,
            "current_stock": r.get::<_, i64>(4)?,
        }))
    }).map(|rows| rows.filter_map(|r| r.ok()).collect::<Vec<_>>()).unwrap_or_default();

    (StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "products": items,
        "timestamp": Local::now().to_rfc3339()
    })))
}

async fn handle_branch_stocks(AxumState(state): AxumState<AppServerState>) -> impl IntoResponse {
    let conn = state.db.conn.lock();
    let mut stmt = match conn.prepare("SELECT id, code, name FROM branches") {
        Ok(s) => s,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))),
    };

    let branches = stmt.query_map([], |r| {
        Ok(serde_json::json!({
            "id": r.get::<_, String>(0)?,
            "code": r.get::<_, String>(1)?,
            "name": r.get::<_, String>(2)?,
        }))
    }).map(|rows| rows.filter_map(|r| r.ok()).collect::<Vec<_>>()).unwrap_or_default();

    (StatusCode::OK, Json(serde_json::json!({
        "status": "success",
        "branches": branches,
        "timestamp": Local::now().to_rfc3339()
    })))
}
