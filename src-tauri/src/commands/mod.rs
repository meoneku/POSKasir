use crate::db::Database;
use crate::models::*;
use crate::server::{ServerInfo, ServerManager};
use chrono::Local;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub fn get_server_status(server: State<'_, ServerManager>) -> Result<ServerInfo, String> {
    Ok(server.get_info())
}

#[tauri::command]
pub async fn start_server(
    server: State<'_, ServerManager>,
    port: Option<u16>,
) -> Result<ServerInfo, String> {
    server.start(port).await
}

#[tauri::command]
pub async fn stop_server(server: State<'_, ServerManager>) -> Result<(), String> {
    server.stop().await
}

#[tauri::command]
pub fn get_store_settings(db: State<'_, Database>) -> Result<Vec<AppSetting>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT setting_key, setting_value, setting_group FROM app_settings ORDER BY setting_group, setting_key")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(AppSetting {
                setting_key: row.get(0)?,
                setting_value: row.get(1)?,
                setting_group: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn update_store_setting(
    db: State<'_, Database>,
    key: String,
    value: String,
) -> Result<(), String> {
    let conn = db.conn.lock();
    conn.execute(
        "UPDATE app_settings SET setting_value = ?1, updated_at = CURRENT_TIMESTAMP WHERE setting_key = ?2",
        params![value, key],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_categories(db: State<'_, Database>) -> Result<Vec<Category>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT id, name, description FROM categories ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_products(db: State<'_, Database>) -> Result<Vec<Product>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT p.id, p.sku, p.barcode, p.name, p.category_id, c.name as category_name,
                    p.min_stock_alert, p.is_active, p.is_taxable, p.tax_id, p.base_unit_name, p.current_stock
             FROM products p
             LEFT JOIN categories c ON p.category_id = c.id
             WHERE p.is_active = 1
             ORDER BY p.name ASC",
        )
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    let rows = stmt
        .query_map([], |row| {
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
        })
        .map_err(|e| e.to_string())?;

    for p in rows {
        let mut prod = p.map_err(|e| e.to_string())?;
        // Ambil unit konversi dan harga
        let mut unit_stmt = conn
            .prepare(
                "SELECT id, product_id, unit_name, multiplier, barcode, retail_price,
                        wholesale_price_1, wholesale_price_2, min_qty_wholesale_1, min_qty_wholesale_2
                 FROM product_units WHERE product_id = ?1 ORDER BY multiplier ASC",
            )
            .map_err(|e| e.to_string())?;

        let unit_rows = unit_stmt
            .query_map(params![prod.id], |r| {
                Ok(ProductUnit {
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
            })
            .map_err(|e| e.to_string())?;

        for u in unit_rows {
            prod.units.push(u.map_err(|e| e.to_string())?);
        }
        products.push(prod);
    }

    Ok(products)
}

#[tauri::command]
pub fn create_product(
    db: State<'_, Database>,
    input: CreateProductInput,
) -> Result<String, String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let product_id = Uuid::new_v4().to_string();
    let initial_stock = input.initial_stock.unwrap_or(0);
    let min_stock = input.min_stock_alert.unwrap_or(5);
    let is_tax = input.is_taxable.unwrap_or(1);

    tx.execute(
        "INSERT INTO products (id, sku, barcode, name, category_id, min_stock_alert, is_taxable, base_unit_name, current_stock)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            product_id,
            input.sku,
            input.barcode,
            input.name,
            input.category_id,
            min_stock,
            is_tax,
            input.base_unit_name,
            initial_stock,
        ],
    )
    .map_err(|e| e.to_string())?;

    let unit_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO product_units (id, product_id, unit_name, multiplier, retail_price, wholesale_price_1, wholesale_price_2)
         VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6)",
        params![
            unit_id,
            product_id,
            input.base_unit_name,
            input.retail_price,
            input.wholesale_price_1,
            input.wholesale_price_2,
        ],
    )
    .map_err(|e| e.to_string())?;

    if initial_stock > 0 {
        let mutation_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_in, ending_stock, note)
             VALUES (?1, ?2, 'opname_adj', 'INIT_STOCK', ?3, ?4, 'Stok Awal Produk')",
            params![mutation_id, product_id, initial_stock, initial_stock],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(product_id)
}

#[tauri::command]
pub fn get_taxes(db: State<'_, Database>) -> Result<Vec<Tax>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT id, name, rate_percent, is_default, is_active FROM taxes WHERE is_active = 1")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Tax {
                id: row.get(0)?,
                name: row.get(1)?,
                rate_percent: row.get(2)?,
                is_default: row.get(3)?,
                is_active: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_payment_methods(db: State<'_, Database>) -> Result<Vec<PaymentMethod>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT id, code, name, category, requires_reference, fee_percentage, is_active, sort_order
             FROM payment_methods WHERE is_active = 1 ORDER BY sort_order ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            Ok(PaymentMethod {
                id: row.get(0)?,
                code: row.get(1)?,
                name: row.get(2)?,
                category: row.get(3)?,
                requires_reference: row.get(4)?,
                fee_percentage: row.get(5)?,
                is_active: row.get(6)?,
                sort_order: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_current_shift(db: State<'_, Database>) -> Result<Option<Shift>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT s.id, s.cashier_id, u.full_name as cashier_name, s.start_time, s.end_time,
                    s.starting_cash, s.expected_ending_cash, s.actual_ending_cash, s.cash_difference, s.status, s.notes
             FROM shifts s
             JOIN users u ON s.cashier_id = u.id
             WHERE s.status = 'open'
             ORDER BY s.start_time DESC LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map([], |r| {
            Ok(Shift {
                id: r.get(0)?,
                cashier_id: r.get(1)?,
                cashier_name: r.get(2)?,
                start_time: r.get(3)?,
                end_time: r.get(4)?,
                starting_cash: r.get(5)?,
                expected_ending_cash: r.get(6)?,
                actual_ending_cash: r.get(7)?,
                cash_difference: r.get(8)?,
                status: r.get(9)?,
                notes: r.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    if let Some(shift) = rows.next() {
        Ok(Some(shift.map_err(|e| e.to_string())?))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub fn start_shift(
    db: State<'_, Database>,
    cashier_id: String,
    starting_cash: f64,
) -> Result<String, String> {
    let conn = db.conn.lock();
    let shift_id = Uuid::new_v4().to_string();
    let now = Local::now().to_rfc3339();

    conn.execute(
        "INSERT INTO shifts (id, cashier_id, start_time, starting_cash, status) VALUES (?1, ?2, ?3, ?4, 'open')",
        params![shift_id, cashier_id, now, starting_cash],
    )
    .map_err(|e| e.to_string())?;

    Ok(shift_id)
}

#[tauri::command]
pub fn close_shift(
    db: State<'_, Database>,
    shift_id: String,
    actual_cash: f64,
    notes: Option<String>,
) -> Result<(), String> {
    let conn = db.conn.lock();
    let now = Local::now().to_rfc3339();

    // Hitung total penjualan tunai pada shift ini
    let mut total_cash_sales: f64 = 0.0;
    let mut stmt = conn
        .prepare(
            "SELECT COALESCE(SUM(p.amount_paid - p.change_due), 0.0)
             FROM payments p
             JOIN transactions t ON p.transaction_id = t.id
             JOIN payment_methods pm ON p.payment_method_id = pm.id
             WHERE t.shift_id = ?1 AND pm.category = 'cash'",
        )
        .map_err(|e| e.to_string())?;

    if let Ok(mut rows) = stmt.query(params![shift_id]) {
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            total_cash_sales = row.get(0).unwrap_or(0.0);
        }
    }

    // Ambil modal awal
    let mut starting_cash: f64 = 0.0;
    let mut shift_stmt = conn
        .prepare("SELECT starting_cash FROM shifts WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    if let Ok(mut rows) = shift_stmt.query(params![shift_id]) {
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            starting_cash = row.get(0).unwrap_or(0.0);
        }
    }

    let expected = starting_cash + total_cash_sales;
    let diff = actual_cash - expected;

    conn.execute(
        "UPDATE shifts SET end_time = ?1, expected_ending_cash = ?2, actual_ending_cash = ?3,
                           cash_difference = ?4, status = 'closed', notes = ?5
         WHERE id = ?6",
        params![now, expected, actual_cash, diff, notes, shift_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn add_cash_movement(
    db: State<'_, Database>,
    shift_id: String,
    r#type: String,
    amount: f64,
    reason: String,
) -> Result<(), String> {
    let conn = db.conn.lock();
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO cash_movements (id, shift_id, type, amount, reason) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, shift_id, r#type, amount, reason],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_transaction(
    db: State<'_, Database>,
    input: CreateTransactionInput,
) -> Result<String, String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let tx_id = Uuid::new_v4().to_string();
    let date_str = Local::now().format("%Y%m%d").to_string();
    let rand_seq = Local::now().format("%H%M%S").to_string();
    let invoice_num = format!("INV/{}/{}", date_str, rand_seq);

    // 1. Simpan header transaksi
    tx.execute(
        "INSERT INTO transactions (id, invoice_number, order_type, shift_id, customer_id, cashier_id,
                                   subtotal, discount_total, tax_mode, tax_base_amount, tax_total, grand_total, payment_status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 'paid')",
        params![
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
    )
    .map_err(|e| e.to_string())?;

    // 2. Simpan item transaksi & potong stok produk atomik
    for item in input.items {
        let item_id = Uuid::new_v4().to_string();
        let base_qty = item.quantity * item.multiplier;

        tx.execute(
            "INSERT INTO transaction_items (id, transaction_id, product_id, unit_id, unit_name, multiplier,
                                           quantity, base_qty, unit_price, discount, is_taxable, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
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
        )
        .map_err(|e| e.to_string())?;

        // Potong stok produk
        tx.execute(
            "UPDATE products SET current_stock = current_stock - ?1 WHERE id = ?2",
            params![base_qty, item.product_id],
        )
        .map_err(|e| e.to_string())?;

        // Catat mutasi stok
        let mutation_id = Uuid::new_v4().to_string();
        let mutation_type = if input.order_type == "wholesale" {
            "sale_wholesale"
        } else {
            "sale_retail"
        };

        tx.execute(
            "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_out, ending_stock, note)
             SELECT ?1, ?2, ?3, ?4, ?5, current_stock, 'Penjualan Nota' FROM products WHERE id = ?2",
            params![mutation_id, item.product_id, mutation_type, invoice_num, base_qty],
        )
        .map_err(|e| e.to_string())?;
    }

    // 3. Simpan rincian pembayaran
    for p in input.payments {
        let pay_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO payments (id, transaction_id, payment_method_id, amount_paid, change_due, reference_number)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                pay_id,
                tx_id,
                p.payment_method_id,
                p.amount_paid,
                input.change_due,
                p.reference_number,
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(invoice_num)
}

#[tauri::command]
pub fn get_recent_transactions(db: State<'_, Database>) -> Result<Vec<TransactionHeader>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.invoice_number, t.order_type, t.cashier_id, u.full_name, c.name,
                    t.subtotal, t.tax_total, t.grand_total, t.payment_status, t.created_at
             FROM transactions t
             JOIN users u ON t.cashier_id = u.id
             LEFT JOIN customers c ON t.customer_id = c.id
             ORDER BY t.created_at DESC LIMIT 20",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |r| {
            Ok(TransactionHeader {
                id: r.get(0)?,
                invoice_number: r.get(1)?,
                order_type: r.get(2)?,
                cashier_id: r.get(3)?,
                cashier_name: r.get(4)?,
                customer_name: r.get(5)?,
                subtotal: r.get(6)?,
                tax_total: r.get(7)?,
                grand_total: r.get(8)?,
                payment_status: r.get(9)?,
                created_at: r.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_customers(db: State<'_, Database>) -> Result<Vec<Customer>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT id, code, name, npwp_nik, phone, address, customer_type, credit_limit, current_debt FROM customers ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |r| {
            Ok(Customer {
                id: r.get(0)?,
                code: r.get(1)?,
                name: r.get(2)?,
                npwp_nik: r.get(3)?,
                phone: r.get(4)?,
                address: r.get(5)?,
                customer_type: r.get(6)?,
                credit_limit: r.get(7)?,
                current_debt: r.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_suppliers(db: State<'_, Database>) -> Result<Vec<Supplier>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT id, name, company_name, npwp, phone, address, payment_terms_days FROM suppliers ORDER BY name ASC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |r| {
            Ok(Supplier {
                id: r.get(0)?,
                name: r.get(1)?,
                company_name: r.get(2)?,
                npwp: r.get(3)?,
                phone: r.get(4)?,
                address: r.get(5)?,
                payment_terms_days: r.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn get_dashboard_stats(db: State<'_, Database>) -> Result<DashboardStats, String> {
    let conn = db.conn.lock();
    let today = Local::now().format("%Y-%m-%d").to_string();

    let mut today_sales = 0.0;
    let mut today_count = 0;
    let mut total_products = 0;
    let mut low_stock = 0;

    let mut tx_stmt = conn
        .prepare("SELECT COALESCE(SUM(grand_total), 0.0), COUNT(id) FROM transactions WHERE date(created_at) = date(?1)")
        .map_err(|e| e.to_string())?;

    if let Ok(mut rows) = tx_stmt.query(params![today]) {
        if let Some(r) = rows.next().map_err(|e| e.to_string())? {
            today_sales = r.get(0).unwrap_or(0.0);
            today_count = r.get(1).unwrap_or(0);
        }
    }

    let mut prod_stmt = conn
        .prepare("SELECT COUNT(id), SUM(CASE WHEN current_stock <= min_stock_alert THEN 1 ELSE 0 END) FROM products WHERE is_active = 1")
        .map_err(|e| e.to_string())?;

    if let Ok(mut rows) = prod_stmt.query([]) {
        if let Some(r) = rows.next().map_err(|e| e.to_string())? {
            total_products = r.get(0).unwrap_or(0);
            low_stock = r.get(1).unwrap_or(0);
        }
    }

    Ok(DashboardStats {
        today_sales,
        today_transactions_count: today_count,
        total_products_count: total_products,
        low_stock_count: low_stock,
    })
}
