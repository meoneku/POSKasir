export interface AppSetting {
  setting_key: string;
  setting_value: string;
  setting_group: string;
}

export interface Category {
  id: string;
  name: string;
  description: string | null;
}

export interface ProductUnit {
  id: string;
  product_id: string;
  unit_name: string;
  multiplier: number;
  barcode: string | null;
  retail_price: number;
  wholesale_price_1: number | null;
  wholesale_price_2: number | null;
  min_qty_wholesale_1: number | null;
  min_qty_wholesale_2: number | null;
}

export interface Product {
  id: string;
  sku: string;
  barcode: string | null;
  name: string;
  category_id: string | null;
  category_name: string | null;
  min_stock_alert: number;
  is_active: number;
  is_taxable: number;
  tax_id: string | null;
  base_unit_name: string;
  current_stock: number;
  units: ProductUnit[];
}

export interface UnitInput {
  unit_name: string;
  multiplier: number;
  barcode?: string;
  retail_price: number;
  wholesale_price_1?: number;
  wholesale_price_2?: number;
  min_qty_wholesale_1?: number;
  min_qty_wholesale_2?: number;
}

export interface CreateProductFullInput {
  sku: string;
  barcode?: string;
  name: string;
  category_id?: string;
  min_stock_alert?: number;
  is_taxable?: number;
  base_unit_name: string;
  initial_stock?: number;
  units: UnitInput[];
}

export interface Tax {
  id: string;
  name: string;
  rate_percent: number;
  is_default: number;
  is_active: number;
}

export interface PaymentMethod {
  id: string;
  code: string;
  name: string;
  category: string;
  requires_reference: number;
  fee_percentage: number;
  is_active: number;
  sort_order: number;
}

export interface Customer {
  id: string;
  code: string;
  name: string;
  npwp_nik: string | null;
  phone: string | null;
  address: string | null;
  customer_type: "retail" | "wholesale" | "vip";
  credit_limit: number;
  current_debt: number;
}

export interface CreateCustomerInput {
  code: string;
  name: string;
  npwp_nik?: string;
  phone?: string;
  address?: string;
  customer_type: "retail" | "wholesale" | "vip";
  credit_limit: number;
}

export interface CustomerDebtPayment {
  id: string;
  customer_id: string;
  customer_name: string;
  transaction_invoice: string | null;
  payment_method_name: string;
  amount: number;
  reference_number?: string;
  notes?: string;
  created_at: string;
}

export interface CreditCheckResult {
  customer_id: string;
  customer_name: string;
  credit_limit: number;
  current_debt: number;
  order_amount: number;
  projected_debt: number;
  is_exceeded: boolean;
  exceeded_by: number;
}

export interface DeliveryOrderItem {
  product_name: string;
  unit_name: string;
  quantity: number;
}

export interface DeliveryOrderDetail {
  transaction_id: string;
  do_number: string;
  invoice_number: string;
  customer_name: string;
  customer_phone?: string;
  customer_address?: string;
  driver_name?: string;
  vehicle_number?: string;
  notes?: string;
  created_at: string;
  items: DeliveryOrderItem[];
  store_info: StoreReceiptInfo;
}

export interface Supplier {
  id: string;
  name: string;
  company_name: string | null;
  npwp: string | null;
  phone: string | null;
  address: string | null;
  payment_terms_days: number;
}

export interface CreateSupplierInput {
  name: string;
  company_name?: string;
  npwp?: string;
  phone?: string;
  address?: string;
  payment_terms_days: number;
}

export interface PurchaseItemInput {
  product_id: string;
  unit_id: string;
  multiplier: number;
  quantity: number;
  unit_cost: number;
  subtotal: number;
}

export interface CreatePurchaseInput {
  supplier_id: string;
  payment_type: "cash" | "credit";
  due_date?: string;
  subtotal: number;
  tax_id?: string;
  tax_amount: number;
  total_amount: number;
  paid_amount: number;
  items: PurchaseItemInput[];
}

export interface PurchaseHeader {
  id: string;
  po_number: string;
  supplier_id: string;
  supplier_name: string;
  status: "draft" | "ordered" | "received" | "cancelled";
  payment_type: "cash" | "credit";
  due_date: string | null;
  subtotal: number;
  tax_amount: number;
  total_amount: number;
  paid_amount: number;
  items_count: number;
  created_at: string;
}

export interface StockAdjustmentInput {
  product_id: string;
  actual_stock: number;
  reason: string;
}

export interface Shift {
  id: string;
  cashier_id: string;
  cashier_name: string | null;
  start_time: string;
  end_time: string | null;
  starting_cash: number;
  expected_ending_cash: number | null;
  actual_ending_cash: number | null;
  cash_difference: number | null;
  status: "open" | "closed";
  notes: string | null;
}

export interface CartItem {
  product: Product;
  selected_unit: ProductUnit;
  quantity: number;
  unit_price: number;
  discount: number;
  subtotal: number;
}

export interface CreateTransactionPayload {
  order_type: "retail" | "wholesale";
  customer_id: string | null;
  cashier_id: string;
  shift_id: string | null;
  subtotal: number;
  discount_total: number;
  tax_mode: "inclusive" | "exclusive" | "none";
  tax_base_amount: number;
  tax_total: number;
  grand_total: number;
  items: {
    product_id: string;
    unit_id: string;
    unit_name: string;
    multiplier: number;
    quantity: number;
    unit_price: number;
    discount: number;
    is_taxable: number;
    subtotal: number;
  }[];
  payments: {
    payment_method_id: string;
    amount_paid: number;
    reference_number?: string;
  }[];
  change_due: number;
  due_date?: string;
}

export interface TransactionHeader {
  id: string;
  invoice_number: string;
  order_type: string;
  cashier_id: string;
  cashier_name: string | null;
  customer_name: string | null;
  subtotal: number;
  tax_total: number;
  grand_total: number;
  payment_status: string;
  due_date?: string | null;
  created_at: string;
}

export interface DashboardStats {
  today_sales: number;
  today_transactions_count: number;
  total_products_count: number;
  low_stock_count: number;
}

export interface ServerInfo {
  is_running: boolean;
  host_ip: string;
  port: number;
  connected_clients: number;
  store_name: string;
}

export interface WsEvent {
  event_type: "STOCK_UPDATED" | "TRANSACTION_CREATED" | "CONNECTED" | "PING";
  payload?: any;
  timestamp: string;
}

export interface StoreReceiptInfo {
  name: string;
  branch: string;
  address: string;
  phone: string;
  npwp: string;
  footer: string;
}

export interface TransactionItemDetail {
  id: string;
  product_name: string;
  unit_name: string;
  quantity: number;
  unit_price: number;
  discount: number;
  subtotal: number;
  is_taxable: number;
}

export interface PaymentDetail {
  payment_method_name: string;
  payment_method_code: string;
  category: string;
  amount_paid: number;
  change_due: number;
  reference_number?: string;
}

export interface TransactionDetail {
  id: string;
  invoice_number: string;
  order_type: string;
  cashier_name: string;
  customer_name?: string;
  customer_phone?: string;
  customer_address?: string;
  customer_npwp_nik?: string;
  due_date?: string;
  subtotal: number;
  discount_total: number;
  tax_mode: string;
  tax_base_amount: number;
  tax_total: number;
  grand_total: number;
  payment_status: string;
  terbilang: string;
  created_at: string;
  items: TransactionItemDetail[];
  payments: PaymentDetail[];
  store_info: StoreReceiptInfo;
  receipt_text_58mm: string;
  receipt_text_80mm: string;
}

export interface PaymentMethodSummary {
  method_name: string;
  transaction_count: number;
  total_amount: number;
}

export interface ShiftReport {
  shift_id: string;
  cashier_name: string;
  start_time: string;
  end_time?: string;
  status: string;
  starting_cash: number;
  cash_sales: number;
  non_cash_sales: number;
  cash_in: number;
  cash_out: number;
  expected_cash: number;
  actual_cash?: number;
  cash_difference?: number;
  total_transactions: number;
  total_sales: number;
  payment_summary: PaymentMethodSummary[];
  notes?: string;
  report_text: string;
}

