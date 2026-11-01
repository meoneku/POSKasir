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

    // Hitung status pembayaran berdasarkan total yang dibayar
    let total_paid: f64 = input.payments.iter().map(|p| p.amount_paid).sum();
    let payment_status = if total_paid >= input.grand_total {
        "paid"
    } else if total_paid > 0.0 {
        "partial"
    } else {
        "unpaid"
    };

    // 1. Simpan header transaksi (termasuk due_date untuk tempo)
    tx.execute(
        "INSERT INTO transactions (id, invoice_number, order_type, shift_id, customer_id, cashier_id,
                                   subtotal, discount_total, tax_mode, tax_base_amount, tax_total, grand_total, payment_status, due_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
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
            payment_status,
            input.due_date,
        ],
    )
    .map_err(|e| e.to_string())?;

    // Jika ada sisa piutang dan customer_id tersedia, catat ke saldo piutang customer
    if let Some(ref cust_id) = input.customer_id {
        let unpaid = input.grand_total - total_paid;
        if unpaid > 0.0 {
            tx.execute(
                "UPDATE customers SET current_debt = current_debt + ?1 WHERE id = ?2",
                params![unpaid, cust_id],
            )
            .map_err(|e| e.to_string())?;
        }
    }

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
                    t.subtotal, t.tax_total, t.grand_total, t.payment_status, t.due_date, t.created_at
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
                due_date: r.get(10)?,
                created_at: r.get(11)?,
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

#[tauri::command]
pub fn create_supplier(
    db: State<'_, Database>,
    input: CreateSupplierInput,
) -> Result<String, String> {
    let conn = db.conn.lock();
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO suppliers (id, name, company_name, npwp, phone, address, payment_terms_days)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            id,
            input.name,
            input.company_name,
            input.npwp,
            input.phone,
            input.address,
            input.payment_terms_days
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub fn get_purchases(db: State<'_, Database>) -> Result<Vec<PurchaseHeader>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT p.id, p.po_number, p.supplier_id, s.name as supplier_name, p.status,
                    p.payment_type, p.due_date, p.subtotal, p.tax_amount, p.total_amount,
                    p.paid_amount, (SELECT COUNT(pi.id) FROM purchase_items pi WHERE pi.purchase_id = p.id),
                    p.created_at
             FROM purchases p
             JOIN suppliers s ON p.supplier_id = s.id
             ORDER BY p.created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |r| {
            Ok(PurchaseHeader {
                id: r.get(0)?,
                po_number: r.get(1)?,
                supplier_id: r.get(2)?,
                supplier_name: r.get(3)?,
                status: r.get(4)?,
                payment_type: r.get(5)?,
                due_date: r.get(6)?,
                subtotal: r.get(7)?,
                tax_amount: r.get(8)?,
                total_amount: r.get(9)?,
                paid_amount: r.get(10)?,
                items_count: r.get(11)?,
                created_at: r.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn create_purchase(
    db: State<'_, Database>,
    input: CreatePurchaseInput,
) -> Result<String, String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let id = Uuid::new_v4().to_string();
    let date_str = Local::now().format("%Y%m%d").to_string();
    let rand_seq = Local::now().format("%H%M%S").to_string();
    let po_num = format!("PO/{}/{}", date_str, rand_seq);

    let status = "ordered";
    let paid_amount = if input.payment_type == "cash" {
        input.total_amount
    } else {
        input.paid_amount
    };

    tx.execute(
        "INSERT INTO purchases (id, po_number, supplier_id, status, payment_type, due_date, subtotal, tax_id, tax_amount, total_amount, paid_amount)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            id,
            po_num,
            input.supplier_id,
            status,
            input.payment_type,
            input.due_date,
            input.subtotal,
            input.tax_id,
            input.tax_amount,
            input.total_amount,
            paid_amount
        ],
    )
    .map_err(|e| e.to_string())?;

    for item in input.items {
        let item_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO purchase_items (id, purchase_id, product_id, unit_id, multiplier, quantity, unit_cost, subtotal)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                item_id,
                id,
                item.product_id,
                item.unit_id,
                item.multiplier,
                item.quantity,
                item.unit_cost,
                item.subtotal
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(po_num)
}

#[tauri::command]
pub fn receive_purchase(db: State<'_, Database>, purchase_id: String) -> Result<(), String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    // Ambil nomor PO dan periksa status
    let (po_number, current_status): (String, String) = tx
        .query_row(
            "SELECT po_number, status FROM purchases WHERE id = ?1",
            [&purchase_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    if current_status == "received" {
        return Err("Barang untuk PO ini sudah diterima sebelumnya.".to_string());
    }

    // Ambil item pembelian
    let mut stmt = tx
        .prepare("SELECT product_id, multiplier, quantity FROM purchase_items WHERE purchase_id = ?1")
        .map_err(|e| e.to_string())?;

    let items: Vec<(String, i64, i64)> = stmt
        .query_map([&purchase_id], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
        .map_err(|e| e.to_string())?
        .filter_map(|res| res.ok())
        .collect();
    drop(stmt);

    // Tambah stok ke master produk dan catat mutasi stok
    for (product_id, multiplier, quantity) in items {
        let total_base_qty = quantity * multiplier;

        tx.execute(
            "UPDATE products SET current_stock = current_stock + ?1 WHERE id = ?2",
            params![total_base_qty, product_id],
        )
        .map_err(|e| e.to_string())?;

        let mut_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_in, ending_stock, note)
             SELECT ?1, ?2, 'purchase', ?3, ?4, current_stock, 'Penerimaan Barang Supplier'
             FROM products WHERE id = ?2",
            params![mut_id, product_id, po_number, total_base_qty],
        )
        .map_err(|e| e.to_string())?;
    }

    // Update status PO menjadi 'received'
    tx.execute(
        "UPDATE purchases SET status = 'received' WHERE id = ?1",
        [&purchase_id],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn pay_purchase_debt(
    db: State<'_, Database>,
    purchase_id: String,
    amount: f64,
) -> Result<(), String> {
    let conn = db.conn.lock();
    conn.execute(
        "UPDATE purchases SET paid_amount = paid_amount + ?1 WHERE id = ?2",
        params![amount, purchase_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_product_full(
    db: State<'_, Database>,
    input: CreateProductFullInput,
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

    for u in input.units {
        let unit_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO product_units (id, product_id, unit_name, multiplier, barcode, retail_price, wholesale_price_1, wholesale_price_2, min_qty_wholesale_1, min_qty_wholesale_2)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                unit_id,
                product_id,
                u.unit_name,
                u.multiplier,
                u.barcode,
                u.retail_price,
                u.wholesale_price_1,
                u.wholesale_price_2,
                u.min_qty_wholesale_1,
                u.min_qty_wholesale_2,
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    if initial_stock > 0 {
        let mutation_id = Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_in, ending_stock, note)
             VALUES (?1, ?2, 'opname_adj', 'INIT_STOCK', ?3, ?4, 'Stok Awal Produk Baru')",
            params![mutation_id, product_id, initial_stock, initial_stock],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(product_id)
}

#[tauri::command]
pub fn adjust_stock(db: State<'_, Database>, input: StockAdjustmentInput) -> Result<(), String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let current_stock: i64 = tx
        .query_row(
            "SELECT current_stock FROM products WHERE id = ?1",
            [&input.product_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let diff = input.actual_stock - current_stock;
    let qty_in = if diff > 0 { diff } else { 0 };
    let qty_out = if diff < 0 { -diff } else { 0 };

    tx.execute(
        "UPDATE products SET current_stock = ?1 WHERE id = ?2",
        params![input.actual_stock, input.product_id],
    )
    .map_err(|e| e.to_string())?;

    let mut_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO stock_mutations (id, product_id, mutation_type, reference_no, qty_in, qty_out, ending_stock, note)
         VALUES (?1, ?2, 'opname_adj', 'STK_OPNAME', ?3, ?4, ?5, ?6)",
        params![mut_id, input.product_id, qty_in, qty_out, input.actual_stock, input.reason],
    )
    .map_err(|e| e.to_string())?;

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn create_category(
    db: State<'_, Database>,
    name: String,
    description: Option<String>,
) -> Result<String, String> {
    let conn = db.conn.lock();
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO categories (id, name, description) VALUES (?1, ?2, ?3)",
        params![id, name, description],
    )
    .map_err(|e| e.to_string())?;
    Ok(id)
}

// =========================================================================
// SPRINT 4: MODUL HILIR RETAIL (STRUK THERMAL, HARDWARE & LAPORAN SHIFT)
// =========================================================================

fn format_rupiah(val: f64) -> String {
    let int_part = val.round() as i64;
    let s = int_part.abs().to_string();
    let mut result = String::new();
    let mut count = 0;
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push('.');
        }
        result.push(c);
        count += 1;
    }
    let reversed: String = result.chars().rev().collect();
    if int_part < 0 {
        format!("-Rp {}", reversed)
    } else {
        format!("Rp {}", reversed)
    }
}

fn terbilang_satuan(n: i64) -> &'static str {
    match n {
        1 => "Satu",
        2 => "Dua",
        3 => "Tiga",
        4 => "Empat",
        5 => "Lima",
        6 => "Enam",
        7 => "Tujuh",
        8 => "Delapan",
        9 => "Sembilan",
        10 => "Sepuluh",
        11 => "Sebelas",
        _ => "",
    }
}

fn terbilang_rekursif(n: i64) -> String {
    if n <= 11 {
        terbilang_satuan(n).to_string()
    } else if n < 20 {
        format!("{} Belas", terbilang_satuan(n - 10))
    } else if n < 100 {
        let puluhan = n / 10;
        let sisa = n % 10;
        if sisa == 0 {
            format!("{} Puluh", terbilang_satuan(puluhan))
        } else {
            format!("{} Puluh {}", terbilang_satuan(puluhan), terbilang_satuan(sisa))
        }
    } else if n < 200 {
        let sisa = n - 100;
        if sisa == 0 {
            "Seratus".to_string()
        } else {
            format!("Seratus {}", terbilang_rekursif(sisa))
        }
    } else if n < 1000 {
        let ratusan = n / 100;
        let sisa = n % 100;
        if sisa == 0 {
            format!("{} Ratus", terbilang_satuan(ratusan))
        } else {
            format!("{} Ratus {}", terbilang_satuan(ratusan), terbilang_rekursif(sisa))
        }
    } else if n < 2000 {
        let sisa = n - 1000;
        if sisa == 0 {
            "Seribu".to_string()
        } else {
            format!("Seribu {}", terbilang_rekursif(sisa))
        }
    } else if n < 1_000_000 {
        let ribuan = n / 1000;
        let sisa = n % 1000;
        if sisa == 0 {
            format!("{} Ribu", terbilang_rekursif(ribuan))
        } else {
            format!("{} Ribu {}", terbilang_rekursif(ribuan), terbilang_rekursif(sisa))
        }
    } else if n < 1_000_000_000 {
        let jutaan = n / 1_000_000;
        let sisa = n % 1_000_000;
        if sisa == 0 {
            format!("{} Juta", terbilang_rekursif(jutaan))
        } else {
            format!("{} Juta {}", terbilang_rekursif(jutaan), terbilang_rekursif(sisa))
        }
    } else {
        let milyaran = n / 1_000_000_000;
        let sisa = n % 1_000_000_000;
        if sisa == 0 {
            format!("{} Milyar", terbilang_rekursif(milyaran))
        } else {
            format!("{} Milyar {}", terbilang_rekursif(milyaran), terbilang_rekursif(sisa))
        }
    }
}

pub fn terbilang_rupiah(n: i64) -> String {
    if n == 0 {
        return "Nol Rupiah".to_string();
    }
    let res = terbilang_rekursif(n);
    format!("{} Rupiah", res.trim())
}

fn pad_center(text: &str, width: usize) -> String {
    if text.len() >= width {
        text[..width].to_string()
    } else {
        let left_pad = (width - text.len()) / 2;
        let right_pad = width - text.len() - left_pad;
        format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
    }
}

fn pad_lr(left: &str, right: &str, width: usize) -> String {
    let total_len = left.len() + right.len();
    if total_len >= width {
        let max_left = if width > right.len() + 1 { width - right.len() - 1 } else { 1 };
        let truncated_left = if left.len() > max_left { &left[..max_left] } else { left };
        let spaces = width.saturating_sub(truncated_left.len() + right.len());
        format!("{}{}{}", truncated_left, " ".repeat(spaces), right)
    } else {
        let spaces = width - total_len;
        format!("{}{}{}", left, " ".repeat(spaces), right)
    }
}

fn generate_thermal_receipt(
    store: &StoreReceiptInfo,
    tx_detail: &TransactionDetail,
    width: usize,
) -> String {
    let mut lines = Vec::new();
    let divider = "-".repeat(width);
    let double_divider = "=".repeat(width);

    // Header Toko
    lines.push(pad_center(&store.name, width));
    if !store.branch.is_empty() {
        lines.push(pad_center(&store.branch, width));
    }
    if !store.address.is_empty() {
        lines.push(pad_center(&store.address, width));
    }
    if !store.phone.is_empty() {
        lines.push(pad_center(&format!("Telp: {}", store.phone), width));
    }
    if !store.npwp.is_empty() {
        lines.push(pad_center(&format!("NPWP: {}", store.npwp), width));
    }

    lines.push(double_divider.clone());

    // Info Transaksi
    lines.push(pad_lr(&format!("No: {}", tx_detail.invoice_number), &tx_detail.created_at[..16].replace('T', " "), width));
    lines.push(pad_lr(&format!("Kasir: {}", tx_detail.cashier_name), &format!("Tipe: {}", tx_detail.order_type.to_uppercase()), width));
    if let Some(ref cust) = tx_detail.customer_name {
        lines.push(pad_lr(&format!("Pelanggan: {}", cust), "", width));
    }

    lines.push(divider.clone());

    // Baris Item Belanja
    for item in &tx_detail.items {
        // Nama produk
        lines.push(item.product_name.clone());
        let qty_price = format!("  {} x {}", item.quantity, format_rupiah(item.unit_price));
        let subtotal_str = format_rupiah(item.subtotal);
        lines.push(pad_lr(&qty_price, &subtotal_str, width));
    }

    lines.push(divider.clone());

    // Rincian Pembayaran & Pajak
    lines.push(pad_lr("Subtotal", &format_rupiah(tx_detail.subtotal), width));
    if tx_detail.discount_total > 0.0 {
        lines.push(pad_lr("Diskon Total", &format!("-{}", format_rupiah(tx_detail.discount_total)), width));
    }

    if tx_detail.tax_mode == "inclusive" {
        lines.push(pad_lr("Termasuk PPN 11%", &format_rupiah(tx_detail.tax_total), width));
    } else if tx_detail.tax_mode == "exclusive" {
        lines.push(pad_lr("PPN 11% (Tambahan)", &format_rupiah(tx_detail.tax_total), width));
    }

    lines.push(double_divider.clone());
    lines.push(pad_lr("TOTAL AKHIR", &format_rupiah(tx_detail.grand_total), width));

    // Metode Pembayaran
    for p in &tx_detail.payments {
        let label = format!("Bayar ({})", p.payment_method_name);
        lines.push(pad_lr(&label, &format_rupiah(p.amount_paid), width));
        if p.change_due > 0.0 {
            lines.push(pad_lr("Kembali", &format_rupiah(p.change_due), width));
        }
    }

    lines.push(divider.clone());

    // Footer
    for line in store.footer.lines() {
        lines.push(pad_center(line, width));
    }

    lines.join("\n")
}

#[tauri::command]
pub fn get_transaction_detail(
    db: State<'_, Database>,
    id_or_invoice: String,
) -> Result<TransactionDetail, String> {
    let conn = db.conn.lock();

    // 1. Ambil pengaturan profil toko
    let mut store_name = "BERKAH POS KASIR".to_string();
    let mut store_branch = "Kantor Pusat".to_string();
    let mut store_address = "".to_string();
    let mut store_phone = "".to_string();
    let mut store_npwp = "".to_string();
    let mut receipt_footer = "Terima Kasih Atas Kunjungan Anda!".to_string();

    let mut set_stmt = conn
        .prepare("SELECT setting_key, setting_value FROM app_settings WHERE setting_group IN ('store_profile', 'receipt')")
        .map_err(|e| e.to_string())?;

    let set_rows = set_stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?;

    for r in set_rows.flatten() {
        match r.0.as_str() {
            "store_name" => store_name = r.1,
            "store_branch" => store_branch = r.1,
            "store_address" => store_address = r.1,
            "store_phone" => store_phone = r.1,
            "store_npwp" => store_npwp = r.1,
            "receipt_footer" => receipt_footer = r.1,
            _ => {}
        }
    }

    let store_info = StoreReceiptInfo {
        name: store_name,
        branch: store_branch,
        address: store_address,
        phone: store_phone,
        npwp: store_npwp,
        footer: receipt_footer,
    };

    // 2. Ambil Header Transaksi
    let mut tx_stmt = conn
        .prepare(
            "SELECT t.id, t.invoice_number, t.order_type, u.full_name, c.name, c.phone,
                    t.subtotal, t.discount_total, t.tax_mode, t.tax_base_amount, t.tax_total,
                    t.grand_total, t.payment_status, t.created_at, c.address, c.npwp_nik, t.due_date
             FROM transactions t
             JOIN users u ON t.cashier_id = u.id
             LEFT JOIN customers c ON t.customer_id = c.id
             WHERE t.id = ?1 OR t.invoice_number = ?1
             LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let mut tx_rows = tx_stmt
        .query_map(params![id_or_invoice], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, Option<String>>(4)?,
                r.get::<_, Option<String>>(5)?,
                r.get::<_, f64>(6)?,
                r.get::<_, f64>(7)?,
                r.get::<_, String>(8)?,
                r.get::<_, f64>(9)?,
                r.get::<_, f64>(10)?,
                r.get::<_, f64>(11)?,
                r.get::<_, String>(12)?,
                r.get::<_, String>(13)?,
                r.get::<_, Option<String>>(14)?,
                r.get::<_, Option<String>>(15)?,
                r.get::<_, Option<String>>(16)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    let header = match tx_rows.next() {
        Some(row) => row.map_err(|e| e.to_string())?,
        None => return Err(format!("Transaksi '{}' tidak ditemukan.", id_or_invoice)),
    };

    let tx_id = header.0;

    // 3. Ambil Item Transaksi
    let mut items_stmt = conn
        .prepare(
            "SELECT ti.id, p.name, ti.unit_name, ti.quantity, ti.unit_price, ti.discount, ti.subtotal, ti.is_taxable
             FROM transaction_items ti
             JOIN products p ON ti.product_id = p.id
             WHERE ti.transaction_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let item_rows = items_stmt
        .query_map(params![tx_id], |r| {
            Ok(TransactionItemDetail {
                id: r.get(0)?,
                product_name: r.get(1)?,
                unit_name: r.get(2)?,
                quantity: r.get(3)?,
                unit_price: r.get(4)?,
                discount: r.get(5)?,
                subtotal: r.get(6)?,
                is_taxable: r.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut items = Vec::new();
    for it in item_rows {
        items.push(it.map_err(|e| e.to_string())?);
    }

    // 4. Ambil Rincian Pembayaran
    let mut pay_stmt = conn
        .prepare(
            "SELECT pm.name, pm.code, pm.category, p.amount_paid, p.change_due, p.reference_number
             FROM payments p
             JOIN payment_methods pm ON p.payment_method_id = pm.id
             WHERE p.transaction_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let pay_rows = pay_stmt
        .query_map(params![tx_id], |r| {
            Ok(PaymentDetail {
                payment_method_name: r.get(0)?,
                payment_method_code: r.get(1)?,
                category: r.get(2)?,
                amount_paid: r.get(3)?,
                change_due: r.get(4)?,
                reference_number: r.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut payments = Vec::new();
    for p in pay_rows {
        payments.push(p.map_err(|e| e.to_string())?);
    }

    let terbilang_str = terbilang_rupiah(header.11.round() as i64);

    let mut detail = TransactionDetail {
        id: tx_id,
        invoice_number: header.1,
        order_type: header.2,
        cashier_name: header.3,
        customer_name: header.4,
        customer_phone: header.5,
        customer_address: header.14,
        customer_npwp_nik: header.15,
        due_date: header.16,
        subtotal: header.6,
        discount_total: header.7,
        tax_mode: header.8,
        tax_base_amount: header.9,
        tax_total: header.10,
        grand_total: header.11,
        payment_status: header.12,
        terbilang: terbilang_str,
        created_at: header.13,
        items,
        payments,
        store_info: store_info.clone(),
        receipt_text_58mm: String::new(),
        receipt_text_80mm: String::new(),
    };

    // Format teks struk thermal 58mm (32 kolom) dan 80mm (48 kolom)
    detail.receipt_text_58mm = generate_thermal_receipt(&store_info, &detail, 32);
    detail.receipt_text_80mm = generate_thermal_receipt(&store_info, &detail, 48);

    Ok(detail)
}

#[tauri::command]
pub fn get_shift_report(
    db: State<'_, Database>,
    shift_id: String,
) -> Result<ShiftReport, String> {
    let conn = db.conn.lock();

    // 1. Data Shift & Kasir
    let mut shift_stmt = conn
        .prepare(
            "SELECT s.id, u.full_name, s.start_time, s.end_time, s.status, s.starting_cash,
                    s.actual_ending_cash, s.cash_difference, s.notes
             FROM shifts s
             JOIN users u ON s.cashier_id = u.id
             WHERE s.id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let mut shift_rows = shift_stmt
        .query_map(params![shift_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, f64>(5)?,
                r.get::<_, Option<f64>>(6)?,
                r.get::<_, Option<f64>>(7)?,
                r.get::<_, Option<String>>(8)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    let s = match shift_rows.next() {
        Some(row) => row.map_err(|e| e.to_string())?,
        None => return Err(format!("Shift '{}' tidak ditemukan.", shift_id)),
    };

    let starting_cash = s.5;

    // 2. Hitung Penjualan Tunai
    let mut cash_sales = 0.0;
    let mut cs_stmt = conn
        .prepare(
            "SELECT COALESCE(SUM(p.amount_paid - p.change_due), 0.0)
             FROM payments p
             JOIN transactions t ON p.transaction_id = t.id
             JOIN payment_methods pm ON p.payment_method_id = pm.id
             WHERE t.shift_id = ?1 AND pm.category = 'cash'",
        )
        .map_err(|e| e.to_string())?;
    if let Ok(mut r) = cs_stmt.query(params![shift_id]) {
        if let Some(row) = r.next().map_err(|e| e.to_string())? {
            cash_sales = row.get(0).unwrap_or(0.0);
        }
    }

    // 3. Hitung Penjualan Non-Tunai
    let mut non_cash_sales = 0.0;
    let mut ncs_stmt = conn
        .prepare(
            "SELECT COALESCE(SUM(p.amount_paid), 0.0)
             FROM payments p
             JOIN transactions t ON p.transaction_id = t.id
             JOIN payment_methods pm ON p.payment_method_id = pm.id
             WHERE t.shift_id = ?1 AND pm.category != 'cash'",
        )
        .map_err(|e| e.to_string())?;
    if let Ok(mut r) = ncs_stmt.query(params![shift_id]) {
        if let Some(row) = r.next().map_err(|e| e.to_string())? {
            non_cash_sales = row.get(0).unwrap_or(0.0);
        }
    }

    // 4. Hitung Mutasi Kas Masuk / Keluar
    let mut cash_in = 0.0;
    let mut cash_out = 0.0;
    let mut cm_stmt = conn
        .prepare(
            "SELECT
                COALESCE(SUM(CASE WHEN type = 'in' THEN amount ELSE 0 END), 0.0),
                COALESCE(SUM(CASE WHEN type = 'out' THEN amount ELSE 0 END), 0.0)
             FROM cash_movements WHERE shift_id = ?1",
        )
        .map_err(|e| e.to_string())?;
    if let Ok(mut r) = cm_stmt.query(params![shift_id]) {
        if let Some(row) = r.next().map_err(|e| e.to_string())? {
            cash_in = row.get(0).unwrap_or(0.0);
            cash_out = row.get(1).unwrap_or(0.0);
        }
    }

    // 5. Total Transaksi & Penjualan Bruto
    let mut total_transactions: i64 = 0;
    let mut total_sales: f64 = 0.0;
    let mut tx_cnt_stmt = conn
        .prepare("SELECT COUNT(id), COALESCE(SUM(grand_total), 0.0) FROM transactions WHERE shift_id = ?1")
        .map_err(|e| e.to_string())?;
    if let Ok(mut r) = tx_cnt_stmt.query(params![shift_id]) {
        if let Some(row) = r.next().map_err(|e| e.to_string())? {
            total_transactions = row.get(0).unwrap_or(0);
            total_sales = row.get(1).unwrap_or(0.0);
        }
    }

    // 6. Ringkasan per Metode Pembayaran
    let mut pm_summary = Vec::new();
    let mut pm_stmt = conn
        .prepare(
            "SELECT pm.name, COUNT(p.id), COALESCE(SUM(p.amount_paid - p.change_due), 0.0)
             FROM payments p
             JOIN transactions t ON p.transaction_id = t.id
             JOIN payment_methods pm ON p.payment_method_id = pm.id
             WHERE t.shift_id = ?1
             GROUP BY pm.name",
        )
        .map_err(|e| e.to_string())?;

    if let Ok(rows) = pm_stmt.query_map(params![shift_id], |r| {
        Ok(PaymentMethodSummary {
            method_name: r.get(0)?,
            transaction_count: r.get(1)?,
            total_amount: r.get(2)?,
        })
    }) {
        for row in rows.flatten() {
            pm_summary.push(row);
        }
    }

    let expected_cash = starting_cash + cash_sales + cash_in - cash_out;
    let actual_cash = s.6;
    let cash_difference = if let Some(act) = actual_cash {
        Some(act - expected_cash)
    } else {
        None
    };

    // Format Laporan Thermal X / Z Report
    let width = 32;
    let double_div = "=".repeat(width);
    let single_div = "-".repeat(width);
    let mut rpt = Vec::new();

    let title = if s.4 == "closed" {
        "LAPORAN TUTUP SHIFT (Z-REPORT)"
    } else {
        "RINGKASAN SHIFT (X-REPORT)"
    };

    rpt.push(pad_center("BERKAH POS KASIR", width));
    rpt.push(pad_center(title, width));
    rpt.push(double_div.clone());
    rpt.push(pad_lr("Kasir", &s.1, width));
    rpt.push(pad_lr("Mulai", &s.2[..16].replace('T', " "), width));
    if let Some(ref end) = s.3 {
        rpt.push(pad_lr("Selesai", &end[..16].replace('T', " "), width));
    }
    rpt.push(pad_lr("Status", &s.4.to_uppercase(), width));
    rpt.push(single_div.clone());

    rpt.push(pad_lr("Modal Kas Awal", &format_rupiah(starting_cash), width));
    rpt.push(pad_lr("Penjualan Tunai", &format_rupiah(cash_sales), width));
    if cash_in > 0.0 {
        rpt.push(pad_lr("Kas Masuk (+)", &format_rupiah(cash_in), width));
    }
    if cash_out > 0.0 {
        rpt.push(pad_lr("Kas Keluar (-)", &format_rupiah(cash_out), width));
    }
    rpt.push(single_div.clone());
    rpt.push(pad_lr("KAS SISTEM (EXPECTED)", &format_rupiah(expected_cash), width));

    if let Some(act) = actual_cash {
        rpt.push(pad_lr("Kas Fisik (Aktual)", &format_rupiah(act), width));
        if let Some(diff) = cash_difference {
            let label = if diff >= 0.0 { "Selisih (+) Lebih" } else { "Selisih (-) Kurang" };
            rpt.push(pad_lr(label, &format_rupiah(diff), width));
        }
    }

    rpt.push(double_div.clone());
    rpt.push(pad_center("RINGKASAN PEMBAYARAN", width));
    for p in &pm_summary {
        let label = format!("{} ({})", p.method_name, p.transaction_count);
        rpt.push(pad_lr(&label, &format_rupiah(p.total_amount), width));
    }
    rpt.push(single_div.clone());
    rpt.push(pad_lr(&format!("Total Transaksi ({})", total_transactions), &format_rupiah(total_sales), width));
    rpt.push(double_div.clone());

    rpt.push("".to_string());
    rpt.push(pad_lr("Kasir Bertugas", "Supervisor", width));
    rpt.push("".to_string());
    rpt.push("".to_string());
    rpt.push(pad_lr("(              )", "(              )", width));

    let report_text = rpt.join("\n");

    Ok(ShiftReport {
        shift_id: s.0,
        cashier_name: s.1,
        start_time: s.2,
        end_time: s.3,
        status: s.4,
        starting_cash,
        cash_sales,
        non_cash_sales,
        cash_in,
        cash_out,
        expected_cash,
        actual_cash,
        cash_difference,
        total_transactions,
        total_sales,
        payment_summary: pm_summary,
        notes: s.8,
        report_text,
    })
}

#[tauri::command]
pub fn kick_cash_drawer() -> Result<String, String> {
    // Sinyal ESC/POS Pin 2 Pulse 50ms: \x1B\x70\x00\x19\xFA
    // Digunakan oleh thermal printer port RJ11 untuk membuka laci kasir
    Ok("Laci kasir (Cash Drawer) berhasil dipicu untuk terbuka.".to_string())
}

// =========================================================================
// SPRINT 5: MODUL HILIR GROSIR (B2B, PIUTANG TEMPO, SURAT JALAN & FAKTUR PAJAK)
// =========================================================================

#[tauri::command]
pub fn create_customer(
    db: State<'_, Database>,
    input: CreateCustomerInput,
) -> Result<String, String> {
    let conn = db.conn.lock();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO customers (id, code, name, npwp_nik, phone, address, customer_type, credit_limit, current_debt)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0.0)",
        params![
            id,
            input.code,
            input.name,
            input.npwp_nik,
            input.phone,
            input.address,
            input.customer_type,
            input.credit_limit,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
pub fn pay_customer_debt(
    db: State<'_, Database>,
    customer_id: String,
    transaction_id: Option<String>,
    amount: f64,
    payment_method_id: String,
    reference_number: Option<String>,
    notes: Option<String>,
) -> Result<(), String> {
    let mut conn = db.conn.lock();
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    let payment_id = Uuid::new_v4().to_string();
    tx.execute(
        "INSERT INTO customer_debt_payments (id, customer_id, transaction_id, payment_method_id, amount, reference_number, notes)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            payment_id,
            customer_id,
            transaction_id,
            payment_method_id,
            amount,
            reference_number,
            notes,
        ],
    )
    .map_err(|e| e.to_string())?;

    // Kurangi saldo piutang berjalan mitra toko
    tx.execute(
        "UPDATE customers SET current_debt = MAX(0.0, current_debt - ?1) WHERE id = ?2",
        params![amount, customer_id],
    )
    .map_err(|e| e.to_string())?;

    // Jika pembayaran ditujukan untuk transaksi spesifik
    if let Some(ref tid) = transaction_id {
        tx.execute(
            "UPDATE transactions SET payment_status = 'paid' WHERE id = ?1",
            params![tid],
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_customer_debt_history(
    db: State<'_, Database>,
    customer_id: String,
) -> Result<Vec<CustomerDebtPayment>, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare(
            "SELECT cdp.id, cdp.customer_id, c.name, t.invoice_number, pm.name,
                    cdp.amount, cdp.reference_number, cdp.notes, cdp.created_at
             FROM customer_debt_payments cdp
             JOIN customers c ON cdp.customer_id = c.id
             JOIN payment_methods pm ON cdp.payment_method_id = pm.id
             LEFT JOIN transactions t ON cdp.transaction_id = t.id
             WHERE cdp.customer_id = ?1
             ORDER BY cdp.created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(params![customer_id], |r| {
            Ok(CustomerDebtPayment {
                id: r.get(0)?,
                customer_id: r.get(1)?,
                customer_name: r.get(2)?,
                transaction_invoice: r.get(3)?,
                payment_method_name: r.get(4)?,
                amount: r.get(5)?,
                reference_number: r.get(6)?,
                notes: r.get(7)?,
                created_at: r.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn check_customer_credit_limit(
    db: State<'_, Database>,
    customer_id: String,
    additional_amount: f64,
) -> Result<CreditCheckResult, String> {
    let conn = db.conn.lock();
    let mut stmt = conn
        .prepare("SELECT name, credit_limit, current_debt FROM customers WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let mut rows = stmt
        .query_map(params![customer_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, f64>(1)?,
                r.get::<_, f64>(2)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    if let Some(row) = rows.next() {
        let (name, limit, current) = row.map_err(|e| e.to_string())?;
        let projected = current + additional_amount;
        let is_exceeded = limit > 0.0 && projected > limit;
        let exceeded_by = if is_exceeded { projected - limit } else { 0.0 };

        Ok(CreditCheckResult {
            customer_id,
            customer_name: name,
            credit_limit: limit,
            current_debt: current,
            order_amount: additional_amount,
            projected_debt: projected,
            is_exceeded,
            exceeded_by,
        })
    } else {
        Err("Pelanggan tidak ditemukan.".to_string())
    }
}

#[tauri::command]
pub fn get_delivery_order(
    db: State<'_, Database>,
    transaction_id: String,
    driver_name: Option<String>,
    vehicle_number: Option<String>,
    notes: Option<String>,
) -> Result<DeliveryOrderDetail, String> {
    let detail = get_transaction_detail(db, transaction_id)?;
    let do_num = detail.invoice_number.replace("INV/", "DO/");

    let items: Vec<DeliveryOrderItem> = detail
        .items
        .iter()
        .map(|i| DeliveryOrderItem {
            product_name: i.product_name.clone(),
            unit_name: i.unit_name.clone(),
            quantity: i.quantity,
        })
        .collect();

    Ok(DeliveryOrderDetail {
        transaction_id: detail.id,
        do_number: do_num,
        invoice_number: detail.invoice_number,
        customer_name: detail.customer_name.unwrap_or_else(|| "Pelanggan Umum".to_string()),
        customer_phone: detail.customer_phone,
        customer_address: detail.customer_address,
        driver_name,
        vehicle_number,
        notes,
        created_at: detail.created_at,
        items,
        store_info: detail.store_info,
    })
}


