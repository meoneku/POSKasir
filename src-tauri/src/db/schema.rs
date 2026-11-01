pub const CREATE_TABLES_SQL: &str = r#"
-- 0. Master Cabang & Multi-Branch
CREATE TABLE IF NOT EXISTS branches (
    id TEXT PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    address TEXT,
    phone TEXT,
    is_headquarters INTEGER DEFAULT 0,
    api_endpoint TEXT,
    api_key TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 1. Pengaturan Toko & Konfigurasi Sistem
CREATE TABLE IF NOT EXISTS app_settings (
    setting_key TEXT PRIMARY KEY,
    setting_value TEXT NOT NULL,
    setting_group TEXT NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Metode Pembayaran
CREATE TABLE IF NOT EXISTS payment_methods (
    id TEXT PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    category TEXT CHECK(category IN ('cash', 'bank_transfer', 'ewallet', 'edc', 'other')),
    requires_reference INTEGER DEFAULT 0,
    fee_percentage REAL DEFAULT 0.0,
    is_active INTEGER DEFAULT 1,
    sort_order INTEGER DEFAULT 0
);

-- 3. Tarif Perpajakan
CREATE TABLE IF NOT EXISTS taxes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    rate_percent REAL NOT NULL,
    is_default INTEGER DEFAULT 0,
    is_active INTEGER DEFAULT 1
);

-- 4. Pengguna & Otorisasi
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    full_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'supervisor', 'cashier', 'warehouse')),
    pin_code TEXT,
    is_active INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 5. Pemasok & Pembelian
CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    company_name TEXT,
    npwp TEXT,
    phone TEXT,
    address TEXT,
    payment_terms_days INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS purchases (
    id TEXT PRIMARY KEY,
    po_number TEXT UNIQUE NOT NULL,
    supplier_id TEXT NOT NULL REFERENCES suppliers(id),
    status TEXT CHECK(status IN ('draft', 'ordered', 'received', 'cancelled')),
    payment_type TEXT CHECK(payment_type IN ('cash', 'credit')),
    due_date DATE,
    subtotal REAL NOT NULL,
    tax_id TEXT REFERENCES taxes(id),
    tax_amount REAL DEFAULT 0.0,
    total_amount REAL NOT NULL,
    paid_amount REAL DEFAULT 0.0,
    created_by TEXT REFERENCES users(id),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS purchase_items (
    id TEXT PRIMARY KEY,
    purchase_id TEXT NOT NULL REFERENCES purchases(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    unit_id TEXT NOT NULL REFERENCES product_units(id),
    multiplier INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    unit_cost REAL NOT NULL,
    subtotal REAL NOT NULL
);

-- 6. Kategori, Produk, Multi-Satuan & Mutasi Stok
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    sku TEXT UNIQUE NOT NULL,
    barcode TEXT UNIQUE,
    name TEXT NOT NULL,
    category_id TEXT REFERENCES categories(id),
    min_stock_alert INTEGER DEFAULT 5,
    is_active INTEGER DEFAULT 1,
    is_taxable INTEGER DEFAULT 1,
    tax_id TEXT REFERENCES taxes(id),
    base_unit_name TEXT NOT NULL,
    current_stock INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS product_units (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    unit_name TEXT NOT NULL,
    multiplier INTEGER NOT NULL,
    barcode TEXT,
    retail_price REAL NOT NULL,
    wholesale_price_1 REAL,
    wholesale_price_2 REAL,
    min_qty_wholesale_1 INTEGER DEFAULT 6,
    min_qty_wholesale_2 INTEGER DEFAULT 24
);

CREATE TABLE IF NOT EXISTS stock_mutations (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id),
    mutation_type TEXT CHECK(mutation_type IN ('purchase', 'sale_retail', 'sale_wholesale', 'return_in', 'return_out', 'opname_adj', 'transfer_in', 'transfer_out')),
    reference_no TEXT,
    qty_in INTEGER DEFAULT 0,
    qty_out INTEGER DEFAULT 0,
    ending_stock INTEGER NOT NULL,
    note TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 7. Pelanggan & Piutang (Wholesale)
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    npwp_nik TEXT,
    phone TEXT,
    address TEXT,
    customer_type TEXT DEFAULT 'retail' CHECK(customer_type IN ('retail', 'wholesale', 'vip')),
    credit_limit REAL DEFAULT 0.0,
    current_debt REAL DEFAULT 0.0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 8. Shift Kasir & Arus Kas
CREATE TABLE IF NOT EXISTS shifts (
    id TEXT PRIMARY KEY,
    cashier_id TEXT NOT NULL REFERENCES users(id),
    start_time DATETIME NOT NULL,
    end_time DATETIME,
    starting_cash REAL NOT NULL,
    expected_ending_cash REAL,
    actual_ending_cash REAL,
    cash_difference REAL,
    status TEXT DEFAULT 'open' CHECK(status IN ('open', 'closed')),
    notes TEXT
);

CREATE TABLE IF NOT EXISTS cash_movements (
    id TEXT PRIMARY KEY,
    shift_id TEXT NOT NULL REFERENCES shifts(id),
    type TEXT CHECK(type IN ('in', 'out')),
    amount REAL NOT NULL,
    reason TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 9. Transaksi Penjualan & Pembayaran
CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    branch_id TEXT REFERENCES branches(id),
    invoice_number TEXT UNIQUE NOT NULL,
    order_type TEXT CHECK(order_type IN ('retail', 'wholesale')),
    shift_id TEXT REFERENCES shifts(id),
    customer_id TEXT REFERENCES customers(id),
    cashier_id TEXT NOT NULL REFERENCES users(id),
    subtotal REAL NOT NULL,
    discount_total REAL DEFAULT 0.0,
    tax_mode TEXT DEFAULT 'inclusive' CHECK(tax_mode IN ('inclusive', 'exclusive', 'none')),
    tax_base_amount REAL NOT NULL,
    tax_total REAL DEFAULT 0.0,
    grand_total REAL NOT NULL,
    payment_status TEXT CHECK(payment_status IN ('paid', 'partial', 'unpaid')),
    due_date DATE,
    sync_status TEXT DEFAULT 'pending' CHECK(sync_status IN ('pending', 'synced')),
    synced_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS transaction_items (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    unit_id TEXT NOT NULL REFERENCES product_units(id),
    unit_name TEXT NOT NULL,
    multiplier INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    base_qty INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    discount REAL DEFAULT 0.0,
    is_taxable INTEGER DEFAULT 1,
    subtotal REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    transaction_id TEXT NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    payment_method_id TEXT NOT NULL REFERENCES payment_methods(id),
    amount_paid REAL NOT NULL,
    change_due REAL DEFAULT 0.0,
    reference_number TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS customer_debt_payments (
    id TEXT PRIMARY KEY,
    customer_id TEXT NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES transactions(id),
    payment_method_id TEXT NOT NULL REFERENCES payment_methods(id),
    amount REAL NOT NULL,
    reference_number TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 10. Mutasi Stok Antar Cabang
CREATE TABLE IF NOT EXISTS inter_branch_transfers (
    id TEXT PRIMARY KEY,
    transfer_number TEXT UNIQUE NOT NULL,
    source_branch_id TEXT NOT NULL REFERENCES branches(id),
    destination_branch_id TEXT NOT NULL REFERENCES branches(id),
    status TEXT CHECK(status IN ('draft', 'in_transit', 'received', 'cancelled')),
    sent_by TEXT REFERENCES users(id),
    received_by TEXT REFERENCES users(id),
    sent_at DATETIME,
    received_at DATETIME,
    sync_status TEXT DEFAULT 'pending' CHECK(sync_status IN ('pending', 'synced')),
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS inter_branch_transfer_items (
    id TEXT PRIMARY KEY,
    transfer_id TEXT NOT NULL REFERENCES inter_branch_transfers(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id),
    unit_id TEXT NOT NULL REFERENCES product_units(id),
    quantity INTEGER NOT NULL,
    base_qty INTEGER NOT NULL
);

-- 11. Riwayat & Log Sinkronisasi Multi-Cabang
CREATE TABLE IF NOT EXISTS sync_logs (
    id TEXT PRIMARY KEY,
    branch_id TEXT REFERENCES branches(id),
    sync_direction TEXT CHECK(sync_direction IN ('push', 'pull', 'two_way')),
    sync_type TEXT DEFAULT 'cloud_relay' CHECK(sync_type IN ('cloud_relay', 'mesh_vpn')),
    records_pushed INTEGER DEFAULT 0,
    records_pulled INTEGER DEFAULT 0,
    status TEXT CHECK(status IN ('success', 'failed', 'partial')),
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
"#;

pub const SEED_DATA_SQL: &str = r#"
-- Inisialisasi Data Default jika kosong

-- Cabang Master Default
INSERT OR IGNORE INTO branches (id, code, name, address, phone, is_headquarters, api_endpoint, api_key) VALUES
('b0000000-0000-0000-0000-000000000001', 'PST-01', 'Toko Kasir Pusat (Jakarta)', 'Jl. Perdagangan Niaga No. 88, Jakarta Pusat', '021-5551234', 1, 'https://sync.tokokasir.id/api/v1', 'KEY-HQ-PST01'),
('b0000000-0000-0000-0000-000000000002', 'SBY-02', 'Cabang Surabaya Gubeng', 'Jl. Pemuda No. 45, Surabaya', '031-7778899', 0, 'http://100.64.0.2:8080', 'KEY-BR-SBY02'),
('b0000000-0000-0000-0000-000000000003', 'BDG-03', 'Cabang Bandung Dago', 'Jl. Ir. H. Juanda No. 112, Bandung', '022-8889900', 0, 'http://100.64.0.3:8080', 'KEY-BR-BDG03');

-- Pengaturan Toko
INSERT OR IGNORE INTO app_settings (setting_key, setting_value, setting_group) VALUES
('store_name', 'BERKAH POS KASIR', 'store_profile'),
('store_branch', 'Kantor Pusat', 'store_profile'),
('store_address', 'Jl. Perdagangan Niaga No. 88, Jakarta', 'store_profile'),
('store_phone', '0812-3456-7890', 'store_profile'),
('store_tagline', 'Belanja Murah, Cepat, dan Amanah', 'store_profile'),
('store_npwp', '01.234.567.8-901.000', 'store_profile'),
('receipt_footer', 'Terima Kasih Atas Kunjungan Anda\nBarang yang sudah dibeli tidak dapat ditukar/dikembalikan.', 'receipt'),
('receipt_paper_width', '58', 'receipt'),
('receipt_auto_cut', '1', 'receipt'),
('receipt_open_drawer', '1', 'receipt'),
('tax_mode_default', 'inclusive', 'tax'),
('app_mode', 'standalone', 'network'), -- 'standalone', 'server', 'client'
('current_branch_id', 'b0000000-0000-0000-0000-000000000001', 'sync'),
('sync_mode', 'cloud_relay', 'sync'),
('cloud_relay_url', 'https://sync.tokokasir.id/api/v1', 'sync'),
('cloud_relay_api_key', 'KEY-HQ-BERKAH-2026', 'sync'),
('mesh_vpn_ip', '100.64.0.1:8080', 'sync'),
('auto_sync_interval', '5', 'sync'),
('last_sync_time', 'Belum pernah', 'sync');

-- Pajak Default
INSERT OR IGNORE INTO taxes (id, name, rate_percent, is_default, is_active) VALUES
('t0000000-0000-0000-0000-000000000001', 'PPN 11%', 11.0, 1, 1),
('t0000000-0000-0000-0000-000000000002', 'PPN 12%', 12.0, 0, 1),
('t0000000-0000-0000-0000-000000000003', 'Bebas Pajak (Non-BKP)', 0.0, 0, 1);

-- Metode Pembayaran
INSERT OR IGNORE INTO payment_methods (id, code, name, category, sort_order) VALUES
('pm000000-0000-0000-0000-000000000001', 'cash', 'Uang Tunai', 'cash', 1),
('pm000000-0000-0000-0000-000000000002', 'qris', 'QRIS Mandiri / BCA', 'ewallet', 2),
('pm000000-0000-0000-0000-000000000003', 'transfer', 'Transfer Bank', 'bank_transfer', 3),
('pm000000-0000-0000-0000-000000000004', 'edc', 'Kartu Debit / EDC', 'edc', 4);

-- Pengguna Default (Admin & Kasir)
INSERT OR IGNORE INTO users (id, username, full_name, password_hash, role, pin_code) VALUES
('u0000000-0000-0000-0000-000000000001', 'admin', 'Administrator Toko', 'admin123', 'admin', '1234'),
('u0000000-0000-0000-0000-000000000002', 'kasir1', 'Kasir Utama (Budi)', 'kasir123', 'cashier', '1111');

-- Kategori Default
INSERT OR IGNORE INTO categories (id, name, description) VALUES
('c0000000-0000-0000-0000-000000000001', 'Sembako', 'Kebutuhan pokok dan bahan makanan'),
('c0000000-0000-0000-0000-000000000002', 'Minuman', 'Minuman ringan, susu, teh, kopi'),
('c0000000-0000-0000-0000-000000000003', 'Perawatan & Kebersihan', 'Sabun, sampo, deterjen');

-- Produk Contoh
INSERT OR IGNORE INTO products (id, sku, barcode, name, category_id, min_stock_alert, is_taxable, base_unit_name, current_stock) VALUES
('p0000000-0000-0000-0000-000000000001', 'MINYAK-01', '899123456701', 'Minyak Goreng 2L', 'c0000000-0000-0000-0000-000000000001', 10, 1, 'Pouch', 48),
('p0000000-0000-0000-0000-000000000002', 'BERAS-01', '899123456702', 'Beras Pandan Wangi 5kg', 'c0000000-0000-0000-0000-000000000001', 5, 0, 'Sak', 25),
('p0000000-0000-0000-0000-000000000003', 'GULA-01', '899123456703', 'Gula Pasir 1kg', 'c0000000-0000-0000-0000-000000000001', 15, 1, 'Bks', 80),
('p0000000-0000-0000-0000-000000000004', 'KOPI-01', '899123456704', 'Kopi Kapal Api Special 165g', 'c0000000-0000-0000-0000-000000000002', 10, 1, 'Bks', 40),
('p0000000-0000-0000-0000-000000000005', 'TEH-01', '899123456705', 'Teh Celup Sosro 30s', 'c0000000-0000-0000-0000-000000000002', 10, 1, 'Kotak', 60),
('p0000000-0000-0000-0000-000000000006', 'SABUN-01', '899123456706', 'Sabun Mandi Batang 85g', 'c0000000-0000-0000-0000-000000000003', 20, 1, 'Pcs', 120);

-- Satuan & Tier Harga Produk
INSERT OR IGNORE INTO product_units (id, product_id, unit_name, multiplier, retail_price, wholesale_price_1, wholesale_price_2) VALUES
('u0000001-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000001', 'Pouch', 1, 35000, 33500, 32000),
('u0000001-0000-0000-0000-000000000002', 'p0000000-0000-0000-0000-000000000001', 'Dus (6 Pouch)', 6, 204000, 198000, 192000),
('u0000002-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000002', 'Sak', 1, 78000, 75000, 72000),
('u0000003-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000003', 'Bks', 1, 17500, 16800, 16000),
('u0000004-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000004', 'Bks', 1, 14500, 13800, 13000),
('u0000005-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000005', 'Kotak', 1, 7500, 7000, 6500),
('u0000006-0000-0000-0000-000000000001', 'p0000000-0000-0000-0000-000000000006', 'Pcs', 1, 4500, 4200, 4000);

-- Pelanggan Contoh
INSERT OR IGNORE INTO customers (id, code, name, phone, customer_type, credit_limit, current_debt) VALUES
('cust0000-0000-0000-0000-000000000001', 'CUST-001', 'Pelanggan Umum', '-', 'retail', 0, 0),
('cust0000-0000-0000-0000-000000000002', 'CUST-002', 'Toko Berkah Sejahtera (Mitra Grosir)', '081399887766', 'wholesale', 15000000, 2500000);

-- Supplier Contoh
INSERT OR IGNORE INTO suppliers (id, name, company_name, phone, address, payment_terms_days) VALUES
('sup00000-0000-0000-0000-000000000001', 'PT. Pangan Nusantara', 'Pangan Nusantara Logistics', '021-5551234', 'Kawasan Industri Pulogadung', 14),
('sup00000-0000-0000-0000-000000000002', 'CV. Sumber Segar Makmur', 'Sumber Segar Wholesale', '081122334455', 'Pasar Induk Kramat Jati', 7);

"#;
