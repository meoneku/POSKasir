import { invoke } from "@tauri-apps/api/core";
import type {
  AppSetting,
  Category,
  Product,
  Tax,
  PaymentMethod,
  Shift,
  Customer,
  Supplier,
  CreateTransactionPayload,
  TransactionHeader,
  DashboardStats,
  ServerInfo,
  WsEvent,
} from "./types";

const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export const api = {
  async getStoreSettings(): Promise<AppSetting[]> {
    if (!isTauri) {
      return [
        { setting_key: "store_name", setting_value: "BERKAH POS KASIR", setting_group: "store_profile" },
        { setting_key: "store_branch", setting_value: "Kantor Pusat", setting_group: "store_profile" },
        { setting_key: "store_phone", setting_value: "0812-3456-7890", setting_group: "store_profile" },
      ];
    }
    return invoke("get_store_settings");
  },

  async updateStoreSetting(key: string, value: string): Promise<void> {
    if (!isTauri) return;
    return invoke("update_store_setting", { key, value });
  },

  async getCategories(): Promise<Category[]> {
    if (!isTauri) {
      return [
        { id: "c1", name: "Sembako", description: "Bahan makanan pokok" },
        { id: "c2", name: "Minuman", description: "Aneka minuman" },
        { id: "c3", name: "Perawatan", description: "Alat mandi & sabun" },
      ];
    }
    return invoke("get_categories");
  },

  async getProducts(): Promise<Product[]> {
    if (!isTauri) {
      return [
        {
          id: "p1",
          sku: "MINYAK-01",
          barcode: "899123456701",
          name: "Minyak Goreng 2L",
          category_id: "c1",
          category_name: "Sembako",
          min_stock_alert: 10,
          is_active: 1,
          is_taxable: 1,
          tax_id: "t1",
          base_unit_name: "Pouch",
          current_stock: 48,
          units: [
            {
              id: "u1",
              product_id: "p1",
              unit_name: "Pouch",
              multiplier: 1,
              barcode: "899123456701",
              retail_price: 35000,
              wholesale_price_1: 33500,
              wholesale_price_2: 32000,
              min_qty_wholesale_1: 6,
              min_qty_wholesale_2: 24,
            },
            {
              id: "u2",
              product_id: "p1",
              unit_name: "Dus (6 Pouch)",
              multiplier: 6,
              barcode: null,
              retail_price: 204000,
              wholesale_price_1: 198000,
              wholesale_price_2: 192000,
              min_qty_wholesale_1: 2,
              min_qty_wholesale_2: 5,
            },
          ],
        },
        {
          id: "p2",
          sku: "BERAS-01",
          barcode: "899123456702",
          name: "Beras Pandan Wangi 5kg",
          category_id: "c1",
          category_name: "Sembako",
          min_stock_alert: 5,
          is_active: 1,
          is_taxable: 0,
          tax_id: null,
          base_unit_name: "Sak",
          current_stock: 25,
          units: [
            {
              id: "u3",
              product_id: "p2",
              unit_name: "Sak",
              multiplier: 1,
              barcode: "899123456702",
              retail_price: 78000,
              wholesale_price_1: 75000,
              wholesale_price_2: 72000,
              min_qty_wholesale_1: 5,
              min_qty_wholesale_2: 20,
            },
          ],
        },
        {
          id: "p3",
          sku: "GULA-01",
          barcode: "899123456703",
          name: "Gula Pasir 1kg",
          category_id: "c1",
          category_name: "Sembako",
          min_stock_alert: 15,
          is_active: 1,
          is_taxable: 1,
          tax_id: "t1",
          base_unit_name: "Bks",
          current_stock: 80,
          units: [
            {
              id: "u4",
              product_id: "p3",
              unit_name: "Bks",
              multiplier: 1,
              barcode: "899123456703",
              retail_price: 17500,
              wholesale_price_1: 16800,
              wholesale_price_2: 16000,
              min_qty_wholesale_1: 10,
              min_qty_wholesale_2: 24,
            },
          ],
        },
      ];
    }
    return invoke("get_products");
  },

  async createProduct(input: any): Promise<string> {
    if (!isTauri) return "mock-id";
    return invoke("create_product", { input });
  },

  async getTaxes(): Promise<Tax[]> {
    if (!isTauri) {
      return [
        { id: "t1", name: "PPN 11%", rate_percent: 11.0, is_default: 1, is_active: 1 },
        { id: "t2", name: "Bebas Pajak", rate_percent: 0.0, is_default: 0, is_active: 1 },
      ];
    }
    return invoke("get_taxes");
  },

  async getPaymentMethods(): Promise<PaymentMethod[]> {
    if (!isTauri) {
      return [
        { id: "pm1", code: "cash", name: "Uang Tunai", category: "cash", requires_reference: 0, fee_percentage: 0, is_active: 1, sort_order: 1 },
        { id: "pm2", code: "qris", name: "QRIS", category: "ewallet", requires_reference: 0, fee_percentage: 0, is_active: 1, sort_order: 2 },
        { id: "pm3", code: "transfer", name: "Transfer Bank", category: "bank_transfer", requires_reference: 1, fee_percentage: 0, is_active: 1, sort_order: 3 },
      ];
    }
    return invoke("get_payment_methods");
  },

  async getCurrentShift(): Promise<Shift | null> {
    if (!isTauri) {
      return {
        id: "s1",
        cashier_id: "u2",
        cashier_name: "Kasir Utama (Budi)",
        start_time: new Date().toISOString(),
        end_time: null,
        starting_cash: 500000,
        expected_ending_cash: null,
        actual_ending_cash: null,
        cash_difference: null,
        status: "open",
        notes: null,
      };
    }
    return invoke("get_current_shift");
  },

  async startShift(cashierId: string, startingCash: number): Promise<string> {
    if (!isTauri) return "shift-id";
    return invoke("start_shift", { cashierId, startingCash });
  },

  async closeShift(shiftId: string, actualCash: number, notes?: string): Promise<void> {
    if (!isTauri) return;
    return invoke("close_shift", { shiftId, actualCash, notes });
  },

  async addCashMovement(shiftId: string, type: "in" | "out", amount: number, reason: string): Promise<void> {
    if (!isTauri) return;
    return invoke("add_cash_movement", { shiftId, type, amount, reason });
  },

  async createTransaction(input: CreateTransactionPayload): Promise<string> {
    if (!isTauri) return "INV/MOCK/001";
    return invoke("create_transaction", { input });
  },

  async getRecentTransactions(): Promise<TransactionHeader[]> {
    if (!isTauri) return [];
    return invoke("get_recent_transactions");
  },

  async getCustomers(): Promise<Customer[]> {
    if (!isTauri) {
      return [
        { id: "c1", code: "CUST-001", name: "Pelanggan Umum", npwp_nik: null, phone: null, address: null, customer_type: "retail", credit_limit: 0, current_debt: 0 },
        { id: "c2", code: "CUST-002", name: "Toko Berkah Sejahtera (Mitra)", npwp_nik: null, phone: "081399887766", address: "Pasar Baru No 12", customer_type: "wholesale", credit_limit: 15000000, current_debt: 2500000 },
      ];
    }
    return invoke("get_customers");
  },

  async getSuppliers(): Promise<Supplier[]> {
    if (!isTauri) {
      return [
        { id: "sup1", name: "PT. Pangan Nusantara", company_name: "Pangan Nusantara", npwp: null, phone: "021-5551234", address: "Jakarta", payment_terms_days: 14 },
      ];
    }
    return invoke("get_suppliers");
  },

  async getDashboardStats(): Promise<DashboardStats> {
    if (!isTauri) {
      return {
        today_sales: 1250000,
        today_transactions_count: 15,
        total_products_count: 6,
        low_stock_count: 1,
      };
    }
    return invoke("get_dashboard_stats");
  },

  async getServerStatus(): Promise<ServerInfo> {
    if (!isTauri) {
      return {
        is_running: true,
        host_ip: "192.168.1.100",
        port: 8080,
        connected_clients: 2,
        store_name: "BERKAH POS KASIR",
      };
    }
    return invoke("get_server_status");
  },

  async startServer(port?: number): Promise<ServerInfo> {
    if (!isTauri) {
      return {
        is_running: true,
        host_ip: "192.168.1.100",
        port: port || 8080,
        connected_clients: 0,
        store_name: "BERKAH POS KASIR",
      };
    }
    return invoke("start_server", { port });
  },

  async stopServer(): Promise<void> {
    if (!isTauri) return;
    return invoke("stop_server");
  },

  initWebSocket(hostIp: string, port: number, onMessage: (event: WsEvent) => void): () => void {
    const wsUrl = `ws://${hostIp || "127.0.0.1"}:${port || 8080}/ws`;
    let ws: WebSocket | null = null;
    let reconnectTimer: any = null;
    let isClosedExplicitly = false;

    const connect = () => {
      try {
        ws = new WebSocket(wsUrl);

        ws.onopen = () => {
          console.log("[WebSocket] Terhubung ke Server Kasir Lokal:", wsUrl);
        };

        ws.onmessage = (e) => {
          try {
            const data: WsEvent = JSON.parse(e.data);
            onMessage(data);
          } catch (err) {
            console.error("[WebSocket] Gagal parse pesan:", err);
          }
        };

        ws.onclose = () => {
          if (!isClosedExplicitly) {
            // Coba sambung ulang setiap 3 detik
            reconnectTimer = setTimeout(connect, 3000);
          }
        };

        ws.onerror = (err) => {
          console.warn("[WebSocket] Koneksi gagal/terputus.");
          ws?.close();
        };
      } catch (e) {
        if (!isClosedExplicitly) {
          reconnectTimer = setTimeout(connect, 3000);
        }
      }
    };

    connect();

    return () => {
      isClosedExplicitly = true;
      clearTimeout(reconnectTimer);
      ws?.close();
    };
  },
};
