use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppSetting {
    pub setting_key: String,
    pub setting_value: String,
    pub setting_group: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductUnit {
    pub id: String,
    pub product_id: String,
    pub unit_name: String,
    pub multiplier: i64,
    pub barcode: Option<String>,
    pub retail_price: f64,
    pub wholesale_price_1: Option<f64>,
    pub wholesale_price_2: Option<f64>,
    pub min_qty_wholesale_1: Option<i64>,
    pub min_qty_wholesale_2: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub min_stock_alert: i64,
    pub is_active: i64,
    pub is_taxable: i64,
    pub tax_id: Option<String>,
    pub base_unit_name: String,
    pub current_stock: i64,
    pub units: Vec<ProductUnit>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitInput {
    pub unit_name: String,
    pub multiplier: i64,
    pub barcode: Option<String>,
    pub retail_price: f64,
    pub wholesale_price_1: Option<f64>,
    pub wholesale_price_2: Option<f64>,
    pub min_qty_wholesale_1: Option<i64>,
    pub min_qty_wholesale_2: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductFullInput {
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub category_id: Option<String>,
    pub min_stock_alert: Option<i64>,
    pub is_taxable: Option<i64>,
    pub base_unit_name: String,
    pub initial_stock: Option<i64>,
    pub units: Vec<UnitInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateProductInput {
    pub sku: String,
    pub barcode: Option<String>,
    pub name: String,
    pub category_id: Option<String>,
    pub min_stock_alert: Option<i64>,
    pub is_taxable: Option<i64>,
    pub base_unit_name: String,
    pub initial_stock: Option<i64>,
    pub retail_price: f64,
    pub wholesale_price_1: Option<f64>,
    pub wholesale_price_2: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tax {
    pub id: String,
    pub name: String,
    pub rate_percent: f64,
    pub is_default: i64,
    pub is_active: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentMethod {
    pub id: String,
    pub code: String,
    pub name: String,
    pub category: String,
    pub requires_reference: i64,
    pub fee_percentage: f64,
    pub is_active: i64,
    pub sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub is_active: i64,
    pub pin_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub token: String,
    pub login_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserAccountInput {
    pub username: String,
    pub full_name: String,
    pub password: Option<String>,
    pub pin_code: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateUserAccountInput {
    pub id: String,
    pub username: String,
    pub full_name: String,
    pub password: Option<String>,
    pub pin_code: Option<String>,
    pub role: String,
    pub is_active: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: String,
    pub code: String,
    pub name: String,
    pub npwp_nik: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub customer_type: String,
    pub credit_limit: f64,
    pub current_debt: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateCustomerInput {
    pub code: String,
    pub name: String,
    pub npwp_nik: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub customer_type: String, // 'retail', 'wholesale', 'vip'
    pub credit_limit: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDebtPayment {
    pub id: String,
    pub customer_id: String,
    pub customer_name: String,
    pub transaction_invoice: Option<String>,
    pub payment_method_name: String,
    pub amount: f64,
    pub reference_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditCheckResult {
    pub customer_id: String,
    pub customer_name: String,
    pub credit_limit: f64,
    pub current_debt: f64,
    pub order_amount: f64,
    pub projected_debt: f64,
    pub is_exceeded: bool,
    pub exceeded_by: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryOrderItem {
    pub product_name: String,
    pub unit_name: String,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeliveryOrderDetail {
    pub transaction_id: String,
    pub do_number: String,
    pub invoice_number: String,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub driver_name: Option<String>,
    pub vehicle_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub items: Vec<DeliveryOrderItem>,
    pub store_info: StoreReceiptInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supplier {
    pub id: String,
    pub name: String,
    pub company_name: Option<String>,
    pub npwp: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms_days: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSupplierInput {
    pub name: String,
    pub company_name: Option<String>,
    pub npwp: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub payment_terms_days: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shift {
    pub id: String,
    pub cashier_id: String,
    pub cashier_name: Option<String>,
    pub start_time: String,
    pub end_time: Option<String>,
    pub starting_cash: f64,
    pub expected_ending_cash: Option<f64>,
    pub actual_ending_cash: Option<f64>,
    pub cash_difference: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CashMovement {
    pub id: String,
    pub shift_id: String,
    pub r#type: String, // 'in' or 'out'
    pub amount: f64,
    pub reason: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItemInput {
    pub product_id: String,
    pub unit_id: String,
    pub unit_name: String,
    pub multiplier: i64,
    pub quantity: i64,
    pub unit_price: f64,
    pub discount: f64,
    pub is_taxable: i64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentInput {
    pub payment_method_id: String,
    pub amount_paid: f64,
    pub reference_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransactionInput {
    pub order_type: String, // 'retail' or 'wholesale'
    pub customer_id: Option<String>,
    pub cashier_id: String,
    pub shift_id: Option<String>,
    pub subtotal: f64,
    pub discount_total: f64,
    pub tax_mode: String, // 'inclusive', 'exclusive', 'none'
    pub tax_base_amount: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub items: Vec<CartItemInput>,
    pub payments: Vec<PaymentInput>,
    pub change_due: f64,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionHeader {
    pub id: String,
    pub invoice_number: String,
    pub order_type: String,
    pub cashier_id: String,
    pub cashier_name: Option<String>,
    pub customer_name: Option<String>,
    pub subtotal: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub payment_status: String,
    pub due_date: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseItemInput {
    pub product_id: String,
    pub unit_id: String,
    pub multiplier: i64,
    pub quantity: i64,
    pub unit_cost: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePurchaseInput {
    pub supplier_id: String,
    pub payment_type: String, // 'cash' or 'credit'
    pub due_date: Option<String>,
    pub subtotal: f64,
    pub tax_id: Option<String>,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub items: Vec<PurchaseItemInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PurchaseHeader {
    pub id: String,
    pub po_number: String,
    pub supplier_id: String,
    pub supplier_name: String,
    pub status: String,
    pub payment_type: String,
    pub due_date: Option<String>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub items_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockAdjustmentInput {
    pub product_id: String,
    pub actual_stock: i64,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardStats {
    pub today_sales: f64,
    pub today_transactions_count: i64,
    pub total_products_count: i64,
    pub low_stock_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreReceiptInfo {
    pub name: String,
    pub branch: String,
    pub address: String,
    pub phone: String,
    pub npwp: String,
    pub footer: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionItemDetail {
    pub id: String,
    pub product_name: String,
    pub unit_name: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub discount: f64,
    pub subtotal: f64,
    pub is_taxable: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentDetail {
    pub payment_method_name: String,
    pub payment_method_code: String,
    pub category: String,
    pub amount_paid: f64,
    pub change_due: f64,
    pub reference_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionDetail {
    pub id: String,
    pub invoice_number: String,
    pub order_type: String,
    pub cashier_name: String,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_address: Option<String>,
    pub customer_npwp_nik: Option<String>,
    pub due_date: Option<String>,
    pub subtotal: f64,
    pub discount_total: f64,
    pub tax_mode: String,
    pub tax_base_amount: f64,
    pub tax_total: f64,
    pub grand_total: f64,
    pub payment_status: String,
    pub terbilang: String,
    pub created_at: String,
    pub items: Vec<TransactionItemDetail>,
    pub payments: Vec<PaymentDetail>,
    pub store_info: StoreReceiptInfo,
    pub receipt_text_58mm: String,
    pub receipt_text_80mm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentMethodSummary {
    pub method_name: String,
    pub transaction_count: i64,
    pub total_amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShiftReport {
    pub shift_id: String,
    pub cashier_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
    pub status: String,
    pub starting_cash: f64,
    pub cash_sales: f64,
    pub non_cash_sales: f64,
    pub cash_in: f64,
    pub cash_out: f64,
    pub expected_cash: f64,
    pub actual_cash: Option<f64>,
    pub cash_difference: Option<f64>,
    pub total_transactions: i64,
    pub total_sales: f64,
    pub payment_summary: Vec<PaymentMethodSummary>,
    pub notes: Option<String>,
    pub report_text: String,
}

// ============================================================================
// SPRINT 6: MULTI-CABANG & SINKRONISASI
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Branch {
    pub id: String,
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_headquarters: i64,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateBranchInput {
    pub code: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_headquarters: i64,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterBranchTransfer {
    pub id: String,
    pub transfer_number: String,
    pub source_branch_id: String,
    pub source_branch_name: String,
    pub destination_branch_id: String,
    pub destination_branch_name: String,
    pub status: String, // 'draft', 'in_transit', 'received', 'cancelled'
    pub sent_by: Option<String>,
    pub sent_by_name: Option<String>,
    pub received_by: Option<String>,
    pub received_by_name: Option<String>,
    pub sent_at: Option<String>,
    pub received_at: Option<String>,
    pub sync_status: String,
    pub notes: Option<String>,
    pub items_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterBranchTransferItem {
    pub id: String,
    pub transfer_id: String,
    pub product_id: String,
    pub product_name: String,
    pub sku: String,
    pub unit_id: String,
    pub unit_name: String,
    pub quantity: i64,
    pub base_qty: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTransferItemInput {
    pub product_id: String,
    pub unit_id: String,
    pub quantity: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateInterBranchTransferInput {
    pub destination_branch_id: String,
    pub sent_by: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<CreateTransferItemInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterBranchTransferDetail {
    pub transfer: InterBranchTransfer,
    pub items: Vec<InterBranchTransferItem>,
    pub source_branch: Branch,
    pub destination_branch: Branch,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BranchStockInfo {
    pub branch_id: String,
    pub branch_code: String,
    pub branch_name: String,
    pub stock_qty: i64,
    pub base_unit_name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub is_current_branch: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncLog {
    pub id: String,
    pub branch_id: Option<String>,
    pub branch_name: Option<String>,
    pub sync_direction: String,
    pub sync_type: String,
    pub records_pushed: i64,
    pub records_pulled: i64,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatusSummary {
    pub pending_transactions: i64,
    pub pending_transfers: i64,
    pub last_sync_time: String,
    pub last_sync_status: String,
    pub current_branch_code: String,
    pub current_branch_name: String,
    pub is_headquarters: bool,
    pub sync_mode: String,
    pub cloud_relay_url: String,
    pub mesh_vpn_ip: String,
}

