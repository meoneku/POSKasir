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

export interface Supplier {
  id: string;
  name: string;
  company_name: string | null;
  npwp: string | null;
  phone: string | null;
  address: string | null;
  payment_terms_days: number;
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
