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
  CreateSupplierInput,
  CreatePurchaseInput,
  PurchaseHeader,
  CreateProductFullInput,
  StockAdjustmentInput,
  TransactionDetail,
  ShiftReport,
  CreateCustomerInput,
  CustomerDebtPayment,
  CreditCheckResult,
  DeliveryOrderDetail,
  Branch,
  CreateBranchInput,
  InterBranchTransfer,
  CreateInterBranchTransferInput,
  InterBranchTransferDetail,
  BranchStockInfo,
  SyncLog,
  SyncStatusSummary,
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

  async createSupplier(input: CreateSupplierInput): Promise<string> {
    if (!isTauri) return "sup-mock";
    return invoke("create_supplier", { input });
  },

  async getPurchases(): Promise<PurchaseHeader[]> {
    if (!isTauri) {
      return [
        {
          id: "po1",
          po_number: "PO/20261101/001",
          supplier_id: "sup1",
          supplier_name: "PT. Pangan Nusantara",
          status: "ordered",
          payment_type: "credit",
          due_date: "2026-11-15",
          subtotal: 3500000,
          tax_amount: 385000,
          total_amount: 3885000,
          paid_amount: 0,
          items_count: 3,
          created_at: new Date().toISOString(),
        },
      ];
    }
    return invoke("get_purchases");
  },

  async createPurchase(input: CreatePurchaseInput): Promise<string> {
    if (!isTauri) return "PO/MOCK/001";
    return invoke("create_purchase", { input });
  },

  async receivePurchase(purchaseId: string): Promise<void> {
    if (!isTauri) return;
    return invoke("receive_purchase", { purchaseId });
  },

  async payPurchaseDebt(purchaseId: string, amount: number): Promise<void> {
    if (!isTauri) return;
    return invoke("pay_purchase_debt", { purchaseId, amount });
  },

  async createProductFull(input: CreateProductFullInput): Promise<string> {
    if (!isTauri) return "prod-mock";
    return invoke("create_product_full", { input });
  },

  async adjustStock(input: StockAdjustmentInput): Promise<void> {
    if (!isTauri) return;
    return invoke("adjust_stock", { input });
  },

  async createCategory(name: string, description?: string): Promise<string> {
    if (!isTauri) return "cat-mock";
    return invoke("create_category", { name, description });
  },

  async getTransactionDetail(idOrInvoice: string): Promise<TransactionDetail> {
    if (!isTauri) {
      return {
        id: "tx-mock",
        invoice_number: idOrInvoice,
        order_type: "retail",
        cashier_name: "Kasir Utama (Budi)",
        customer_name: "Pelanggan Umum",
        customer_phone: undefined,
        customer_address: "Jl. Melati No. 45, Jakarta",
        customer_npwp_nik: "09.123.456.7-001.000",
        due_date: undefined,
        subtotal: 70000,
        discount_total: 0,
        tax_mode: "inclusive",
        tax_base_amount: 63063,
        tax_total: 6937,
        grand_total: 70000,
        payment_status: "paid",
        terbilang: "Tujuh Puluh Ribu Rupiah",
        created_at: new Date().toISOString(),
        items: [
          {
            id: "ti-1",
            product_name: "Minyak Goreng 2L",
            unit_name: "Pouch",
            quantity: 2,
            unit_price: 35000,
            discount: 0,
            subtotal: 70000,
            is_taxable: 1,
          },
        ],
        payments: [
          {
            payment_method_name: "Uang Tunai",
            payment_method_code: "cash",
            category: "cash",
            amount_paid: 100000,
            change_due: 30000,
          },
        ],
        store_info: {
          name: "BERKAH POS KASIR",
          branch: "Kantor Pusat",
          address: "Jl. Perdagangan Niaga No. 88, Jakarta",
          phone: "0812-3456-7890",
          npwp: "01.234.567.8-901.000",
          footer: "Terima Kasih Atas Kunjungan Anda!\nBarang yang sudah dibeli tidak dapat ditukar/dikembalikan.",
        },
        receipt_text_58mm: "BERKAH POS KASIR\nKantor Pusat\n--------------------------------\nNo: " + idOrInvoice + "\nKasir: Kasir Utama\n--------------------------------\nMinyak Goreng 2L\n  2 x Rp 35.000        Rp 70.000\n--------------------------------\nTOTAL AKHIR            Rp 70.000\nBayar (Tunai)         Rp 100.000\nKembali                Rp 30.000\n--------------------------------\nTerima Kasih Atas Kunjungan Anda!",
        receipt_text_80mm: "BERKAH POS KASIR\nKantor Pusat\n------------------------------------------------\nNo: " + idOrInvoice + "\nKasir: Kasir Utama\n------------------------------------------------\nMinyak Goreng 2L\n  2 x Rp 35.000                                Rp 70.000\n------------------------------------------------\nTOTAL AKHIR                                    Rp 70.000\nBayar (Tunai)                                 Rp 100.000\nKembali                                        Rp 30.000\n------------------------------------------------\nTerima Kasih Atas Kunjungan Anda!",
      };
    }
    return invoke("get_transaction_detail", { idOrInvoice });
  },

  async getShiftReport(shiftId: string): Promise<ShiftReport> {
    if (!isTauri) {
      return {
        shift_id: shiftId,
        cashier_name: "Kasir Utama (Budi)",
        start_time: new Date().toISOString(),
        end_time: undefined,
        status: "open",
        starting_cash: 500000,
        cash_sales: 1250000,
        non_cash_sales: 450000,
        cash_in: 0,
        cash_out: 50000,
        expected_cash: 1700000,
        actual_cash: undefined,
        cash_difference: undefined,
        total_transactions: 15,
        total_sales: 1700000,
        payment_summary: [
          { method_name: "Uang Tunai", transaction_count: 10, total_amount: 1250000 },
          { method_name: "QRIS", transaction_count: 5, total_amount: 450000 },
        ],
        notes: undefined,
        report_text: "BERKAH POS KASIR\nRINGKASAN SHIFT (X-REPORT)\n================================\nKasir: Kasir Utama\nModal Kas Awal: Rp 500.000\nPenjualan Tunai: Rp 1.250.000\nKas Keluar: -Rp 50.000\n--------------------------------\nKAS SISTEM: Rp 1.700.000\n================================",
      };
    }
    return invoke("get_shift_report", { shiftId });
  },

  async kickCashDrawer(): Promise<string> {
    if (!isTauri) return "Laci kasir berhasil dipicu.";
    return invoke("kick_cash_drawer");
  },

  async createCustomer(input: CreateCustomerInput): Promise<string> {
    if (!isTauri) return "cust-mock";
    return invoke("create_customer", { input });
  },

  async payCustomerDebt(
    customerId: string,
    transactionId: string | null,
    amount: number,
    paymentMethodId: string,
    referenceNumber?: string,
    notes?: string
  ): Promise<void> {
    if (!isTauri) return;
    return invoke("pay_customer_debt", {
      customerId,
      transactionId,
      amount,
      paymentMethodId,
      referenceNumber,
      notes,
    });
  },

  async getCustomerDebtHistory(customerId: string): Promise<CustomerDebtPayment[]> {
    if (!isTauri) return [];
    return invoke("get_customer_debt_history", { customerId });
  },

  async checkCustomerCreditLimit(
    customerId: string,
    additionalAmount: number
  ): Promise<CreditCheckResult> {
    if (!isTauri) {
      return {
        customer_id: customerId,
        customer_name: "Toko Berkah Sejahtera",
        credit_limit: 15000000,
        current_debt: 2500000,
        order_amount: additionalAmount,
        projected_debt: 2500000 + additionalAmount,
        is_exceeded: 2500000 + additionalAmount > 15000000,
        exceeded_by: Math.max(0, 2500000 + additionalAmount - 15000000),
      };
    }
    return invoke("check_customer_credit_limit", { customerId, additionalAmount });
  },

  async getDeliveryOrder(
    transactionId: string,
    driverName?: string,
    vehicleNumber?: string,
    notes?: string
  ): Promise<DeliveryOrderDetail> {
    if (!isTauri) {
      return {
        transaction_id: transactionId,
        do_number: "DO/20261101/001",
        invoice_number: "INV/20261101/001",
        customer_name: "Toko Berkah Sejahtera",
        customer_phone: "081399887766",
        customer_address: "Pasar Baru No 12, Jakarta Pusat",
        driver_name: driverName || "Pak Joko",
        vehicle_number: vehicleNumber || "B 1234 XYZ",
        notes: notes || "Harap sertakan stempel toko saat penerimaan",
        created_at: new Date().toISOString(),
        items: [
          { product_name: "Minyak Goreng 2L", unit_name: "Dus (6 Pouch)", quantity: 10 },
          { product_name: "Beras Pandan Wangi 5kg", unit_name: "Sak", quantity: 20 },
        ],
        store_info: {
          name: "BERKAH POS KASIR",
          branch: "Kantor Pusat",
          address: "Jl. Perdagangan Niaga No. 88, Jakarta",
          phone: "0812-3456-7890",
          npwp: "01.234.567.8-901.000",
          footer: "",
        },
      };
    }
    return invoke("get_delivery_order", {
      transactionId,
      driverName,
      vehicleNumber,
      notes,
    });
  },

  // --- SPRINT 6: MULTI-CABANG & SINKRONISASI ---

  async getBranches(): Promise<Branch[]> {
    if (!isTauri) {
      return [
        { id: "b1", code: "PST-01", name: "Toko Kasir Pusat (Jakarta)", address: "Jl. Perdagangan Niaga No. 88, Jakarta", phone: "021-5551234", is_headquarters: 1, created_at: new Date().toISOString() },
        { id: "b2", code: "SBY-02", name: "Cabang Surabaya Gubeng", address: "Jl. Pemuda No. 45, Surabaya", phone: "031-7778899", is_headquarters: 0, created_at: new Date().toISOString() },
        { id: "b3", code: "BDG-03", name: "Cabang Bandung Dago", address: "Jl. Ir. H. Juanda No. 112, Bandung", phone: "022-8889900", is_headquarters: 0, created_at: new Date().toISOString() },
      ];
    }
    return invoke("get_branches");
  },

  async createBranch(input: CreateBranchInput): Promise<string> {
    if (!isTauri) return "branch-new-id";
    return invoke("create_branch", { input });
  },

  async getCurrentBranch(): Promise<Branch> {
    if (!isTauri) {
      return { id: "b1", code: "PST-01", name: "Toko Kasir Pusat (Jakarta)", address: "Jl. Perdagangan Niaga No. 88, Jakarta", phone: "021-5551234", is_headquarters: 1, created_at: new Date().toISOString() };
    }
    return invoke("get_current_branch");
  },

  async setCurrentBranch(branchId: string): Promise<void> {
    if (!isTauri) return;
    return invoke("set_current_branch", { branchId });
  },

  async getInterBranchTransfers(): Promise<InterBranchTransfer[]> {
    if (!isTauri) return [];
    return invoke("get_inter_branch_transfers");
  },

  async getInterBranchTransferDetail(transferId: string): Promise<InterBranchTransferDetail> {
    if (!isTauri) {
      return {
        transfer: {
          id: transferId,
          transfer_number: "TRF/PST-SBY/202611/0001",
          source_branch_id: "b1",
          source_branch_name: "Toko Kasir Pusat (Jakarta)",
          destination_branch_id: "b2",
          destination_branch_name: "Cabang Surabaya Gubeng",
          status: "in_transit",
          sent_by: "u1",
          sent_by_name: "Admin Pusat",
          received_by: undefined,
          received_by_name: undefined,
          sent_at: new Date().toISOString(),
          received_at: undefined,
          sync_status: "synced",
          notes: "Kiriman stok tambahan minyak dan beras",
          items_count: 2,
          created_at: new Date().toISOString(),
        },
        items: [
          { id: "tfi1", transfer_id: transferId, product_id: "p1", product_name: "Minyak Goreng 2L", sku: "MINYAK-01", unit_id: "u1", unit_name: "Dus (6 Pouch)", quantity: 10, base_qty: 60 },
          { id: "tfi2", transfer_id: transferId, product_id: "p2", product_name: "Beras Pandan Wangi 5kg", sku: "BERAS-01", unit_id: "u2", unit_name: "Sak", quantity: 20, base_qty: 20 },
        ],
        source_branch: { id: "b1", code: "PST-01", name: "Toko Kasir Pusat (Jakarta)", address: "Jl. Perdagangan Niaga No. 88, Jakarta", phone: "021-5551234", is_headquarters: 1, created_at: new Date().toISOString() },
        destination_branch: { id: "b2", code: "SBY-02", name: "Cabang Surabaya Gubeng", address: "Jl. Pemuda No. 45, Surabaya", phone: "031-7778899", is_headquarters: 0, created_at: new Date().toISOString() },
      };
    }
    return invoke("get_inter_branch_transfer_detail", { transferId });
  },

  async createInterBranchTransfer(input: CreateInterBranchTransferInput): Promise<string> {
    if (!isTauri) return "TRF/PST-SBY/202611/0001";
    return invoke("create_inter_branch_transfer", { input });
  },

  async receiveInterBranchTransfer(transferId: string, receivedBy: string): Promise<void> {
    if (!isTauri) return;
    return invoke("receive_inter_branch_transfer", { transferId, receivedBy });
  },

  async getCrossBranchStock(productId: string): Promise<BranchStockInfo[]> {
    if (!isTauri) {
      return [
        { branch_id: "b1", branch_code: "PST-01", branch_name: "Toko Kasir Pusat (Jakarta)", stock_qty: 48, base_unit_name: "Pcs", is_current_branch: true },
        { branch_id: "b2", branch_code: "SBY-02", branch_name: "Cabang Surabaya Gubeng", stock_qty: 24, base_unit_name: "Pcs", is_current_branch: false },
        { branch_id: "b3", branch_code: "BDG-03", branch_name: "Cabang Bandung Dago", stock_qty: 18, base_unit_name: "Pcs", is_current_branch: false },
      ];
    }
    return invoke("get_cross_branch_stock", { productId });
  },

  async getSyncStatus(): Promise<SyncStatusSummary> {
    if (!isTauri) {
      return {
        pending_transactions: 0,
        pending_transfers: 0,
        last_sync_time: "Baru saja",
        last_sync_status: "synced",
        current_branch_code: "PST-01",
        current_branch_name: "Toko Kasir Pusat (Jakarta)",
        is_headquarters: true,
        sync_mode: "cloud_relay",
        cloud_relay_url: "https://sync.tokokasir.id/api/v1",
        mesh_vpn_ip: "100.64.0.1:8080",
      };
    }
    return invoke("get_sync_status");
  },

  async triggerBranchSync(syncType?: string): Promise<SyncLog> {
    if (!isTauri) {
      return {
        id: "sync-mock",
        sync_direction: "two_way",
        sync_type: syncType || "cloud_relay",
        records_pushed: 5,
        records_pulled: 6,
        status: "success",
        created_at: new Date().toISOString(),
      };
    }
    return invoke("trigger_branch_sync", { syncType });
  },

  async getSyncLogs(): Promise<SyncLog[]> {
    if (!isTauri) return [];
    return invoke("get_sync_logs");
  },

  async updateSyncSettings(
    syncMode: string,
    cloudRelayUrl: string,
    cloudRelayApiKey: string,
    meshVpnIp: string,
    autoSyncInterval: string
  ): Promise<void> {
    if (!isTauri) return;
    return invoke("update_sync_settings", {
      syncMode,
      cloudRelayUrl,
      cloudRelayApiKey,
      meshVpnIp,
      autoSyncInterval,
    });
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
