<script lang="ts">
  import { onMount } from "svelte";
  import {
    Search,
    Barcode,
    Plus,
    Minus,
    Trash2,
    PauseCircle,
    PlayCircle,
    CheckCircle,
    Boxes,
    PackagePlus,
    FileSpreadsheet,
    Printer,
    Truck,
    Building2,
    Users,
    ReceiptText,
    Percent,
    AlertTriangle,
    Wifi,
  } from "@lucide/svelte";

  import Header from "$lib/components/Header.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import Skeleton from "$lib/components/Skeleton.svelte";
  import PaymentModal from "$lib/components/PaymentModal.svelte";
  import ShiftModal from "$lib/components/ShiftModal.svelte";
  import AddProductModal from "$lib/components/AddProductModal.svelte";
  import CreatePOModal from "$lib/components/CreatePOModal.svelte";
  import AddSupplierModal from "$lib/components/AddSupplierModal.svelte";
  import StockOpnameModal from "$lib/components/StockOpnameModal.svelte";
  import ReceiptModal from "$lib/components/ReceiptModal.svelte";
  import HeldCartsModal from "$lib/components/HeldCartsModal.svelte";
  import EditQtyModal from "$lib/components/EditQtyModal.svelte";
  import AddCustomerModal from "$lib/components/AddCustomerModal.svelte";
  import CustomerDebtModal from "$lib/components/CustomerDebtModal.svelte";
  import WholesaleInvoiceModal from "$lib/components/WholesaleInvoiceModal.svelte";
  import DeliveryOrderModal from "$lib/components/DeliveryOrderModal.svelte";

  import { api } from "$lib/api";
  import type {
    AppSetting,
    Category,
    Product,
    ProductUnit,
    Tax,
    PaymentMethod,
    Shift,
    Customer,
    Supplier,
    CartItem,
    TransactionHeader,
    DashboardStats,
    ServerInfo,
    PurchaseHeader,
    CreatePurchaseInput,
    CreateSupplierInput,
    CreateProductFullInput,
    StockAdjustmentInput,
    TransactionDetail,
    CreateCustomerInput,
    DeliveryOrderDetail,
  } from "$lib/types";

  // --- REAKTIF STATE (SVELTE 5 RUNES) ---
  let activeTab = $state<"pos" | "wholesale" | "inventory" | "procurement" | "customers" | "reports" | "settings">("pos");
  let isLoading = $state(true);
  let toastMessage = $state<string | null>(null);

  // Data Master
  let settings = $state<AppSetting[]>([]);
  let categories = $state<Category[]>([]);
  let products = $state<Product[]>([]);
  let taxes = $state<Tax[]>([]);
  let paymentMethods = $state<PaymentMethod[]>([]);
  let currentShift = $state<Shift | null>(null);
  let customers = $state<Customer[]>([]);
  let suppliers = $state<Supplier[]>([]);
  let purchases = $state<PurchaseHeader[]>([]);
  let unpaidPurchases = $derived(
    purchases.filter((p) => p.payment_type === "credit" && p.total_amount > p.paid_amount)
  );
  let recentTransactions = $state<TransactionHeader[]>([]);
  let dashboardStats = $state<DashboardStats | null>(null);
  let serverInfo = $state<ServerInfo | null>(null);
  let serverPortInput = $state(8080);
  let networkRole = $state<"standalone" | "server" | "client">("server");
  let clientRemoteHost = $state("192.168.1.100:8080");

  // State Kasir Retail & Keranjang
  let searchQuery = $state("");
  let selectedCategoryId = $state<string | null>(null);
  let cart = $state<CartItem[]>([]);
  let selectedCartIndex = $state<number>(0);
  let heldCarts = $state<{ id: string; name: string; items: CartItem[]; time: string }[]>([]);
  let selectedWholesaleCustomerId = $state<string | null>(null);
  let selectedWholesaleCustomer = $derived(
    customers.find((c) => c.id === selectedWholesaleCustomerId) || null
  );

  // Modals & Subtabs State
  let isPaymentModalOpen = $state(false);
  let isShiftModalOpen = $state(false);
  let isAddProductModalOpen = $state(false);
  let isCreatePOModalOpen = $state(false);
  let isAddSupplierModalOpen = $state(false);
  let isStockOpnameModalOpen = $state(false);
  let isReceiptModalOpen = $state(false);
  let activeReceiptDetail = $state<TransactionDetail | null>(null);
  let isHeldCartsModalOpen = $state(false);
  let isEditQtyModalOpen = $state(false);
  let isAddCustomerModalOpen = $state(false);
  let isCustomerDebtModalOpen = $state(false);
  let selectedDebtCustomer = $state<Customer | null>(null);
  let isWholesaleInvoiceModalOpen = $state(false);
  let isDeliveryOrderModalOpen = $state(false);
  let procurementSubTab = $state<"po" | "suppliers" | "debt">("po");
  let customerSubTab = $state<"list" | "receivables">("list");

  let wholesaleRecentTransactions = $derived(
    recentTransactions.filter((t) => t.order_type === "wholesale")
  );
  let unpaidCustomerList = $derived(
    customers.filter((c) => c.current_debt > 0)
  );

  // Input Field References
  let searchInputRef = $state<HTMLInputElement | null>(null);

  // Hitungan Pajak & Keranjang ($derived)
  let storeName = $derived(
    settings.find((s) => s.setting_key === "store_name")?.setting_value || "BERKAH POS KASIR"
  );
  let storeBranch = $derived(
    settings.find((s) => s.setting_key === "store_branch")?.setting_value || "Kantor Pusat"
  );
  let taxMode = $derived<"inclusive" | "exclusive" | "none">(
    (settings.find((s) => s.setting_key === "tax_mode_default")?.setting_value as any) || "inclusive"
  );

  let cartSubtotal = $derived(
    cart.reduce((sum, item) => sum + item.subtotal, 0)
  );

  let taxableSubtotal = $derived(
    cart.reduce((sum, item) => (item.product.is_taxable ? sum + item.subtotal : sum), 0)
  );

  let taxCalculated = $derived(() => {
    if (taxMode === "inclusive") {
      // PPN 11% Termasuk: DPP = Total / 1.11, PPN = Total - DPP
      const dpp = taxableSubtotal / 1.11;
      const ppn = taxableSubtotal - dpp;
      return { dpp, ppn, grandTotal: cartSubtotal };
    } else if (taxMode === "exclusive") {
      // PPN 11% Tambahan
      const ppn = taxableSubtotal * 0.11;
      return { dpp: taxableSubtotal, ppn, grandTotal: cartSubtotal + ppn };
    }
    return { dpp: cartSubtotal, ppn: 0, grandTotal: cartSubtotal };
  });

  let grandTotal = $derived(taxCalculated().grandTotal);
  let taxBase = $derived(taxCalculated().dpp);
  let taxTotal = $derived(taxCalculated().ppn);

  // Filter Produk Katalog
  let filteredProducts = $derived(
    products.filter((p) => {
      const matchCat = !selectedCategoryId || p.category_id === selectedCategoryId;
      const matchSearch =
        !searchQuery.trim() ||
        p.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        p.sku.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (p.barcode && p.barcode.includes(searchQuery));
      return matchCat && matchSearch;
    })
  );

  // --- AUDIO SYNTHESIZER UNTUK BEEP KASIR / SCANNER ---
  function playScannerBeep() {
    try {
      const AudioCtx = window.AudioContext || (window as any).webkitAudioContext;
      if (!AudioCtx) return;
      const ctx = new AudioCtx();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = "sine";
      osc.frequency.setValueAtTime(1760, ctx.currentTime); // 1760Hz crisp tone
      gain.gain.setValueAtTime(0.12, ctx.currentTime);
      gain.gain.exponentialRampToValueAtTime(0.001, ctx.currentTime + 0.08);
      osc.connect(gain);
      gain.connect(ctx.destination);
      osc.start();
      osc.stop(ctx.currentTime + 0.08);
    } catch (e) {
      // Abaikan jika audio tidak didukung
    }
  }

  function handleBarcodeScan(code: string) {
    const clean = code.trim();
    if (!clean) return;

    // 1. Cari kecocokan barcode produk atau SKU
    for (const p of products) {
      if (p.barcode === clean || p.sku.toLowerCase() === clean.toLowerCase()) {
        addToCart(p, p.units[0]);
        playScannerBeep();
        showToast(`Scan Barcode: ${p.name}`);
        return;
      }
      // 2. Cari kecocokan barcode satuan bertingkat (misal barcode kemasan Dus/Pak)
      for (const u of p.units) {
        if (u.barcode === clean) {
          addToCart(p, u);
          playScannerBeep();
          showToast(`Scan Barcode (${u.unit_name}): ${p.name}`);
          return;
        }
      }
    }

    showToast(`Barcode tidak ditemukan: ${clean}`);
  }

  // --- LIFECYCLE & GLOBAL LISTENERS ---
  onMount(() => {
    loadAllData();

    let barcodeBuffer = "";
    let lastKeyTime = 0;

    // Global Barcode & Shortcut Listener
    const handleGlobalKeydown = (e: KeyboardEvent) => {
      const isAnyModalOpen =
        isPaymentModalOpen ||
        isShiftModalOpen ||
        isAddProductModalOpen ||
        isCreatePOModalOpen ||
        isAddSupplierModalOpen ||
        isStockOpnameModalOpen ||
        isReceiptModalOpen ||
        isHeldCartsModalOpen ||
        isEditQtyModalOpen ||
        isAddCustomerModalOpen ||
        isCustomerDebtModalOpen ||
        isWholesaleInvoiceModalOpen ||
        isDeliveryOrderModalOpen;

      // ESC: Menutup dialog aktif
      if (e.key === "Escape") {
        if (isReceiptModalOpen) {
          isReceiptModalOpen = false;
          return;
        }
        if (isHeldCartsModalOpen) {
          isHeldCartsModalOpen = false;
          return;
        }
        if (isEditQtyModalOpen) {
          isEditQtyModalOpen = false;
          return;
        }
        if (isAddCustomerModalOpen) {
          isAddCustomerModalOpen = false;
          return;
        }
        if (isCustomerDebtModalOpen) {
          isCustomerDebtModalOpen = false;
          return;
        }
        if (isWholesaleInvoiceModalOpen) {
          isWholesaleInvoiceModalOpen = false;
          return;
        }
        if (isDeliveryOrderModalOpen) {
          isDeliveryOrderModalOpen = false;
          return;
        }
      }

      // Keyboard Shortcuts kasir hanya aktif saat tidak ada modal terbuka
      if (!isAnyModalOpen) {
        if (e.key === "F2") {
          e.preventDefault();
          activeTab = "pos";
          searchInputRef?.focus();
          searchInputRef?.select();
          return;
        } else if (e.key === "F4") {
          e.preventDefault();
          if (cart.length > 0) {
            selectedCartIndex = Math.min(selectedCartIndex, cart.length - 1);
            isEditQtyModalOpen = true;
          } else {
            showToast("Keranjang belanja masih kosong!");
          }
          return;
        } else if (e.key === "F8") {
          e.preventDefault();
          holdCurrentCart();
          return;
        } else if (e.key === "F9") {
          e.preventDefault();
          isHeldCartsModalOpen = true;
          return;
        } else if (e.key === "F12") {
          e.preventDefault();
          if (cart.length > 0) {
            isPaymentModalOpen = true;
          } else {
            showToast("Keranjang belanja masih kosong!");
          }
          return;
        }
      }

      // Deteksi Kecepatan Input Hardware Barcode Scanner USB HID
      // Scanner barcode mengetik karakter berturut-turut dalam interval < 45 milidetik diakhiri Enter
      const now = Date.now();
      const diff = now - lastKeyTime;
      lastKeyTime = now;

      const isInputActive =
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA";

      if (e.key === "Enter") {
        if (barcodeBuffer.length >= 3 && (diff < 50 || !isInputActive)) {
          e.preventDefault();
          handleBarcodeScan(barcodeBuffer);
          barcodeBuffer = "";
          return;
        }
        barcodeBuffer = "";
      } else if (e.key.length === 1) {
        if (diff > 60 && barcodeBuffer.length > 0) {
          barcodeBuffer = "";
        }
        barcodeBuffer += e.key;
      }
    };

    window.addEventListener("keydown", handleGlobalKeydown);
    return () => window.removeEventListener("keydown", handleGlobalKeydown);
  });

  async function loadAllData() {
    isLoading = true;
    try {
      const [s, c, p, t, pm, sh, cust, sup, r, ds, srv, po] = await Promise.all([
        api.getStoreSettings(),
        api.getCategories(),
        api.getProducts(),
        api.getTaxes(),
        api.getPaymentMethods(),
        api.getCurrentShift(),
        api.getCustomers(),
        api.getSuppliers(),
        api.getRecentTransactions(),
        api.getDashboardStats(),
        api.getServerStatus(),
        api.getPurchases(),
      ]);
      settings = s;
      categories = c;
      products = p;
      taxes = t;
      paymentMethods = pm;
      currentShift = sh;
      customers = cust;
      suppliers = sup;
      recentTransactions = r;
      dashboardStats = ds;
      serverInfo = srv;
      purchases = po;
      if (srv) {
        serverPortInput = srv.port;
      }
    } catch (err) {
      console.error("Gagal memuat data:", err);
      showToast("Gagal memuat basis data!");
    } finally {
      // Transisi halus dengan durasi 150ms
      setTimeout(() => {
        isLoading = false;
      }, 150);
    }
  }

  // --- HANDLER MODUL HULU & INVENTARIS (SPRINT 3) ---
  async function handleCreateProductFull(input: CreateProductFullInput) {
    try {
      await api.createProductFull(input);
      products = await api.getProducts();
      dashboardStats = await api.getDashboardStats();
      isAddProductModalOpen = false;
      showToast(`Produk ${input.name} berhasil ditambahkan!`);
    } catch (err) {
      showToast(`Gagal menambah produk: ${err}`);
    }
  }

  async function handleCreateSupplier(input: CreateSupplierInput) {
    try {
      await api.createSupplier(input);
      suppliers = await api.getSuppliers();
      isAddSupplierModalOpen = false;
      showToast(`Pemasok ${input.name} berhasil disimpan!`);
    } catch (err) {
      showToast(`Gagal menambah supplier: ${err}`);
    }
  }

  async function handleCreatePurchase(input: CreatePurchaseInput) {
    try {
      const poNum = await api.createPurchase(input);
      purchases = await api.getPurchases();
      isCreatePOModalOpen = false;
      showToast(`Purchase Order ${poNum} berhasil diterbitkan!`);
    } catch (err) {
      showToast(`Gagal membuat PO: ${err}`);
    }
  }

  async function handleReceivePurchase(purchaseId: string) {
    try {
      await api.receivePurchase(purchaseId);
      purchases = await api.getPurchases();
      products = await api.getProducts();
      dashboardStats = await api.getDashboardStats();
      showToast("Barang PO berhasil diterima! Stok gudang otomatis bertambah.");
    } catch (err) {
      showToast(`Gagal menerima PO: ${err}`);
    }
  }

  async function handlePayPurchaseDebt(purchaseId: string, amount: number) {
    try {
      await api.payPurchaseDebt(purchaseId, amount);
      purchases = await api.getPurchases();
      showToast(`Pembayaran hutang Rp ${amount.toLocaleString("id-ID")} berhasil dicatat!`);
    } catch (err) {
      showToast(`Gagal membayar hutang: ${err}`);
    }
  }

  async function handleAdjustStock(input: StockAdjustmentInput) {
    try {
      await api.adjustStock(input);
      products = await api.getProducts();
      dashboardStats = await api.getDashboardStats();
      isStockOpnameModalOpen = false;
      showToast("Penyesuaian stok opname berhasil disimpan!");
    } catch (err) {
      showToast(`Gagal opname stok: ${err}`);
    }
  }

  // WebSocket Listener untuk Sinkronisasi Realtime LAN
  $effect(() => {
    if (serverInfo?.is_running && serverInfo.host_ip) {
      const cleanup = api.initWebSocket(serverInfo.host_ip, serverInfo.port, (event) => {
        if (event.event_type === "STOCK_UPDATED") {
          const updates = event.payload?.stock_updates || [];
          for (const u of updates) {
            const prod = products.find((p) => p.id === u.product_id);
            if (prod) {
              prod.current_stock = u.new_stock;
            }
          }
          showToast(`Stok terupdate via sinkronisasi LAN (#${event.payload?.invoice_number || ""})`);
        }
      });
      return cleanup;
    }
  });

  async function handleToggleServer() {
    try {
      if (serverInfo?.is_running) {
        await api.stopServer();
        serverInfo = await api.getServerStatus();
        showToast("Embedded Server Kasir Lokal dihentikan.");
      } else {
        serverInfo = await api.startServer(serverPortInput);
        showToast(`Server Lokal aktif di ${serverInfo.host_ip}:${serverInfo.port}`);
      }
    } catch (err) {
      showToast(`Gagal mengatur server: ${err}`);
    }
  }

  async function handleTestPing() {
    try {
      const srv = await api.getServerStatus();
      serverInfo = srv;
      if (srv.is_running) {
        showToast(`Server Aktif! IP: ${srv.host_ip}:${srv.port} (${srv.connected_clients} klien)`);
      } else {
        showToast("Server saat ini berstatus Non-aktif.");
      }
    } catch (err) {
      showToast("Gagal menghubungi server lokal.");
    }
  }

  function showToast(msg: string) {
    toastMessage = msg;
    setTimeout(() => {
      toastMessage = null;
    }, 2500);
  }

  function handleTabChange(tab: any) {
    isLoading = true;
    activeTab = tab;
    setTimeout(() => {
      isLoading = false;
    }, 150);
  }

  // --- LOGIKA KERANJANG KASIR ---
  function addToCart(product: Product, unit?: ProductUnit) {
    const selectedUnit = unit || product.units[0];
    const existingIndex = cart.findIndex(
      (item) => item.product.id === product.id && item.selected_unit.id === selectedUnit.id
    );

    if (existingIndex >= 0) {
      const updated = [...cart];
      updated[existingIndex].quantity += 1;
      updated[existingIndex].subtotal =
        updated[existingIndex].quantity * updated[existingIndex].unit_price;
      cart = updated;
    } else {
      let price = selectedUnit.retail_price;
      // Cek apakah mode grosir untuk menggunakan tier harga grosir
      if (activeTab === "wholesale" && selectedUnit.wholesale_price_1) {
        price = selectedUnit.wholesale_price_1;
      }

      cart = [
        ...cart,
        {
          product,
          selected_unit: selectedUnit,
          quantity: 1,
          unit_price: price,
          discount: 0,
          subtotal: price,
        },
      ];
    }
  }

  function updateQuantity(index: number, delta: number) {
    const updated = [...cart];
    const newQty = updated[index].quantity + delta;
    if (newQty <= 0) {
      removeFromCart(index);
      return;
    }
    updated[index].quantity = newQty;

    // Otomatis cek tier harga grosir berdasarkan kuantitas
    const unit = updated[index].selected_unit;
    if (unit.min_qty_wholesale_2 && newQty >= unit.min_qty_wholesale_2 && unit.wholesale_price_2) {
      updated[index].unit_price = unit.wholesale_price_2;
    } else if (unit.min_qty_wholesale_1 && newQty >= unit.min_qty_wholesale_1 && unit.wholesale_price_1) {
      updated[index].unit_price = unit.wholesale_price_1;
    } else {
      updated[index].unit_price = unit.retail_price;
    }

    updated[index].subtotal = updated[index].quantity * updated[index].unit_price;
    cart = updated;
  }

  function changeCartUnit(index: number, newUnitId: string) {
    const updated = [...cart];
    const unit = updated[index].product.units.find((u) => u.id === newUnitId);
    if (unit) {
      updated[index].selected_unit = unit;
      updated[index].unit_price = unit.retail_price;
      updated[index].subtotal = updated[index].quantity * unit.retail_price;
      cart = updated;
    }
  }

  function removeFromCart(index: number) {
    cart = cart.filter((_, i) => i !== index);
  }

  function clearCart() {
    cart = [];
  }

  function holdCurrentCart() {
    if (cart.length === 0) return;
    const holdName = `Nota #${heldCarts.length + 1} (${new Date().toLocaleTimeString("id-ID", { hour: "2-digit", minute: "2-digit" })})`;
    heldCarts = [
      ...heldCarts,
      {
        id: crypto.randomUUID(),
        name: holdName,
        items: [...cart],
        time: new Date().toLocaleTimeString("id-ID"),
      },
    ];
    cart = [];
    showToast(`Transaksi ditahan sebagai ${holdName}`);
  }

  function recallLastHeldCart() {
    if (heldCarts.length === 0) {
      showToast("Tidak ada nota yang sedang ditahan.");
      return;
    }
    const last = heldCarts[heldCarts.length - 1];
    cart = [...last.items];
    heldCarts = heldCarts.slice(0, heldCarts.length - 1);
    showToast(`Nota '${last.name}' berhasil dimuat kembali!`);
  }

  function handleRecallHeldCart(cartId: string) {
    const target = heldCarts.find((c) => c.id === cartId);
    if (target) {
      cart = [...target.items];
      heldCarts = heldCarts.filter((c) => c.id !== cartId);
      isHeldCartsModalOpen = false;
      showToast(`Nota '${target.name}' dimuat kembali!`);
    }
  }

  function handleDeleteHeldCart(cartId: string) {
    heldCarts = heldCarts.filter((c) => c.id !== cartId);
    showToast("Nota tertahan dibatalkan.");
  }

  function handleSaveEditedQty(newQty: number) {
    if (selectedCartIndex >= 0 && selectedCartIndex < cart.length) {
      const updated = [...cart];
      updated[selectedCartIndex].quantity = newQty;
      const unit = updated[selectedCartIndex].selected_unit;
      if (unit.min_qty_wholesale_2 && newQty >= unit.min_qty_wholesale_2 && unit.wholesale_price_2) {
        updated[selectedCartIndex].unit_price = unit.wholesale_price_2;
      } else if (unit.min_qty_wholesale_1 && newQty >= unit.min_qty_wholesale_1 && unit.wholesale_price_1) {
        updated[selectedCartIndex].unit_price = unit.wholesale_price_1;
      } else {
        updated[selectedCartIndex].unit_price = unit.retail_price;
      }
      updated[selectedCartIndex].subtotal = updated[selectedCartIndex].quantity * updated[selectedCartIndex].unit_price;
      cart = updated;
      isEditQtyModalOpen = false;
      showToast(`Jumlah diubah menjadi ${newQty} ${unit.unit_name}`);
    }
  }

  async function openReceiptForTransaction(idOrInvoice: string) {
    try {
      const detail = await api.getTransactionDetail(idOrInvoice);
      activeReceiptDetail = detail;
      isReceiptModalOpen = true;
    } catch (err) {
      showToast(`Gagal memuat struk: ${err}`);
    }
  }

  async function openWholesaleInvoice(idOrInvoice: string) {
    try {
      const detail = await api.getTransactionDetail(idOrInvoice);
      activeReceiptDetail = detail;
      isWholesaleInvoiceModalOpen = true;
    } catch (err) {
      showToast(`Gagal memuat faktur A4: ${err}`);
    }
  }

  async function openDeliveryOrder(idOrInvoice: string) {
    try {
      const detail = await api.getTransactionDetail(idOrInvoice);
      activeReceiptDetail = detail;
      isDeliveryOrderModalOpen = true;
    } catch (err) {
      showToast(`Gagal memuat surat jalan: ${err}`);
    }
  }

  // --- SPRINT 5: HANDLER MITRA GROSIR & PIUTANG ---
  async function handleCreateCustomer(input: CreateCustomerInput) {
    try {
      await api.createCustomer(input);
      customers = await api.getCustomers();
      isAddCustomerModalOpen = false;
      showToast(`Pelanggan '${input.name}' berhasil didaftarkan!`);
    } catch (err) {
      showToast(`Gagal menambah pelanggan: ${err}`);
    }
  }

  function handleOpenDebtModal(cust: Customer) {
    selectedDebtCustomer = cust;
    isCustomerDebtModalOpen = true;
  }

  async function handleCustomerPaymentSuccess() {
    isCustomerDebtModalOpen = false;
    customers = await api.getCustomers();
    recentTransactions = await api.getRecentTransactions();
    dashboardStats = await api.getDashboardStats();
    showToast("Pelunasan piutang mitra toko berhasil disimpan!");
  }

  // --- TRANSAKSI CHECKOUT ---
  async function handleConfirmPayment(methodId: string, amountPaid: number, changeDue: number, dueDate?: string) {
    try {
      const isWholesale = activeTab === "wholesale";
      const payload = {
        order_type: (isWholesale ? "wholesale" : "retail") as "retail" | "wholesale",
        customer_id: selectedWholesaleCustomerId,
        cashier_id: currentShift?.cashier_id || "u0000000-0000-0000-0000-000000000002",
        shift_id: currentShift?.id || null,
        subtotal: cartSubtotal,
        discount_total: 0,
        tax_mode: taxMode,
        tax_base_amount: taxBase,
        tax_total: taxTotal,
        grand_total: grandTotal,
        items: cart.map((i) => ({
          product_id: i.product.id,
          unit_id: i.selected_unit.id,
          unit_name: i.selected_unit.unit_name,
          multiplier: i.selected_unit.multiplier,
          quantity: i.quantity,
          unit_price: i.unit_price,
          discount: i.discount,
          is_taxable: i.product.is_taxable,
          subtotal: i.subtotal,
        })),
        payments: amountPaid > 0 ? [
          {
            payment_method_id: methodId,
            amount_paid: amountPaid,
          },
        ] : [],
        change_due: changeDue,
        due_date: dueDate,
      };

      const invoiceNum = await api.createTransaction(payload);
      isPaymentModalOpen = false;
      cart = [];
      showToast(`Transaksi ${invoiceNum} Berhasil Disimpan!`);

      // Buka pratinjau dokumen sesuai tipe penjualan
      try {
        const detail = await api.getTransactionDetail(invoiceNum);
        activeReceiptDetail = detail;
        if (isWholesale) {
          isWholesaleInvoiceModalOpen = true; // Langsung tampilkan Faktur A4 untuk Grosir B2B
        } else {
          isReceiptModalOpen = true; // Struk thermal untuk Retail
        }
      } catch (e) {
        console.warn("Gagal membuka dokumen otomatis:", e);
      }

      // Refresh data
      products = await api.getProducts();
      customers = await api.getCustomers();
      recentTransactions = await api.getRecentTransactions();
      dashboardStats = await api.getDashboardStats();
    } catch (err) {
      console.error("Gagal memproses transaksi:", err);
      showToast("Gagal menyimpan transaksi!");
    }
  }

  // --- MANAJEMEN SHIFT KASIR ---
  async function handleStartShift(startingCash: number) {
    try {
      await api.startShift("u0000000-0000-0000-0000-000000000002", startingCash);
      currentShift = await api.getCurrentShift();
      isShiftModalOpen = false;
      showToast("Shift kasir baru berhasil dibuka!");
    } catch (err) {
      showToast("Gagal membuka shift!");
    }
  }

  async function handleCloseShift(actualCash: number, notes?: string) {
    if (!currentShift) return;
    try {
      await api.closeShift(currentShift.id, actualCash, notes);
      currentShift = null;
      isShiftModalOpen = false;
      showToast("Shift kasir berhasil ditutup!");
    } catch (err) {
      showToast("Gagal menutup shift!");
    }
  }

  async function handleAddCashMovement(type: "in" | "out", amount: number, reason: string) {
    if (!currentShift) return;
    try {
      await api.addCashMovement(currentShift.id, type, amount, reason);
      isShiftModalOpen = false;
      showToast(`Kas ${type === "in" ? "Masuk" : "Keluar"} Rp ${amount.toLocaleString("id-ID")} berhasil dicatat!`);
    } catch (err) {
      showToast("Gagal mencatat mutasi kas!");
    }
  }
</script>

<div class="h-screen flex flex-col bg-slate-50 overflow-hidden font-sans select-none">
  <!-- Toast Notification -->
  {#if toastMessage}
    <div class="fixed top-4 right-4 z-50 bg-slate-900 text-white border border-slate-700 px-4 py-2.5 rounded-lg shadow-lg text-xs font-semibold flex items-center gap-2">
      <CheckCircle class="w-4 h-4 text-emerald-400" />
      <span>{toastMessage}</span>
    </div>
  {/if}

  <!-- Header Navigasi Atas -->
  <Header
    {storeName}
    branchName={storeBranch}
    shift={currentShift}
    {serverInfo}
    onOpenShiftModal={() => (isShiftModalOpen = true)}
  />

  <div class="flex-1 flex overflow-hidden">
    <!-- Sidebar Menu Kiri -->
    <Sidebar {activeTab} onChangeTab={handleTabChange} />

    <!-- Konten Area Kerja Utama -->
    <main class="flex-1 flex flex-col overflow-hidden bg-slate-100">
      {#if isLoading}
        <!-- Efek Skeleton Loader Snappy (150ms) -->
        <div class="p-4 flex-1 overflow-auto">
          {#if activeTab === "pos" || activeTab === "inventory"}
            <Skeleton type="grid" count={8} />
          {:else if activeTab === "reports"}
            <Skeleton type="card" count={4} />
            <div class="mt-4"><Skeleton type="table" rows={6} cols={5} /></div>
          {:else}
            <Skeleton type="table" rows={8} cols={5} />
          {/if}
        </div>
      {:else}
        <!-- TAB 1: KASIR RETAIL (PENJUALAN ECERAN CEPAT) -->
        {#if activeTab === "pos"}
          <div class="flex-1 flex overflow-hidden">
            <!-- Kolom Kiri: Katalog & Pencarian Produk -->
            <div class="flex-1 flex flex-col border-r border-slate-200 bg-slate-50 overflow-hidden">
              <!-- Toolbar Pencarian & Filter Kategori -->
              <div class="p-3 bg-white border-b border-slate-200 space-y-2">
                <div class="relative">
                  <Search class="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
                  <input
                    bind:this={searchInputRef}
                    type="text"
                    placeholder="Cari Nama Produk, SKU, atau Scan Barcode (F2)..."
                    bind:value={searchQuery}
                    onkeydown={(e) => {
                      if (e.key === "Enter" && filteredProducts.length === 1) {
                        addToCart(filteredProducts[0]);
                        searchQuery = "";
                      }
                    }}
                    class="w-full pl-9 pr-4 py-2 bg-slate-100 border border-slate-300 rounded-lg text-xs text-slate-900 placeholder:text-slate-400 focus:outline-none focus:bg-white focus:border-emerald-600 focus:ring-1 focus:ring-emerald-600"
                  />
                  {#if searchQuery}
                    <button
                      onclick={() => (searchQuery = "")}
                      class="absolute right-3 top-1/2 -translate-y-1/2 text-xs text-slate-400 hover:text-slate-600"
                    >
                      Hapus
                    </button>
                  {/if}
                </div>

                <!-- Filter Kategori Kancing (Pills) -->
                <div class="flex gap-1.5 overflow-x-auto pb-1 text-xs">
                  <button
                    onclick={() => (selectedCategoryId = null)}
                    class="px-3 py-1 rounded-full whitespace-nowrap font-medium transition-colors {!selectedCategoryId ? 'bg-emerald-600 text-white font-semibold' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
                  >
                    Semua Kategori
                  </button>
                  {#each categories as cat}
                    <button
                      onclick={() => (selectedCategoryId = cat.id)}
                      class="px-3 py-1 rounded-full whitespace-nowrap font-medium transition-colors {selectedCategoryId === cat.id ? 'bg-emerald-600 text-white font-semibold' : 'bg-slate-100 text-slate-700 hover:bg-slate-200'}"
                    >
                      {cat.name}
                    </button>
                  {/each}
                </div>
              </div>

              <!-- Grid Katalog Produk -->
              <div class="flex-1 p-3 overflow-y-auto">
                {#if filteredProducts.length === 0}
                  <div class="h-full flex flex-col items-center justify-center text-slate-400 text-xs">
                    <Boxes class="w-10 h-10 mb-2 stroke-1" />
                    <span>Tidak ada produk yang cocok dengan pencarian</span>
                  </div>
                {:else}
                  <div class="grid grid-cols-2 sm:grid-cols-3 xl:grid-cols-4 gap-2.5">
                    {#each filteredProducts as prod}
                      {@const defaultUnit = prod.units[0]}
                      <button
                        type="button"
                        onclick={() => addToCart(prod, defaultUnit)}
                        class="bg-white border border-slate-200 hover:border-emerald-600 hover:shadow-xs active:scale-[0.98] rounded-lg p-3 text-left flex flex-col justify-between transition-all group"
                      >
                        <div>
                          <div class="flex justify-between items-start gap-1 mb-1">
                            <span class="text-[10px] font-mono text-slate-400">{prod.sku}</span>
                            {#if prod.is_taxable}
                              <span class="text-[9px] bg-emerald-50 text-emerald-700 border border-emerald-200 px-1 rounded font-semibold">PPN</span>
                            {:else}
                              <span class="text-[9px] bg-slate-100 text-slate-500 border border-slate-200 px-1 rounded">NON</span>
                            {/if}
                          </div>
                          <h4 class="font-bold text-slate-900 text-xs line-clamp-2 leading-snug group-hover:text-emerald-700">
                            {prod.name}
                          </h4>
                          <div class="text-[11px] text-slate-500 mt-0.5">
                            Stok: <span class="font-semibold {prod.current_stock <= prod.min_stock_alert ? 'text-amber-600' : 'text-slate-700'}">{prod.current_stock} {prod.base_unit_name}</span>
                          </div>
                        </div>

                        <div class="mt-3 pt-2 border-t border-slate-100 flex justify-between items-baseline">
                          <span class="text-xs font-bold font-mono text-emerald-700">
                            Rp {defaultUnit?.retail_price.toLocaleString("id-ID")}
                          </span>
                          <span class="text-[10px] text-slate-400">/{defaultUnit?.unit_name}</span>
                        </div>
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Kolom Kanan: Keranjang Belanja Kasir -->
            <div class="w-96 bg-white flex flex-col justify-between overflow-hidden shadow-xs border-l border-slate-200">
              <!-- Header Keranjang & Nota Tertahan -->
              <div class="px-4 py-3 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
                <div class="flex items-center gap-2">
                  <h3 class="font-bold text-slate-900 text-xs uppercase tracking-wider">Keranjang Nota</h3>
                  <span class="text-[11px] bg-emerald-100 text-emerald-800 font-bold px-1.5 py-0.2 rounded-full">
                    {cart.reduce((s, i) => s + i.quantity, 0)}
                  </span>
                </div>
                {#if cart.length > 0}
                  <button
                    onclick={clearCart}
                    class="text-xs text-rose-600 hover:text-rose-700 font-semibold"
                  >
                    Batal Semua
                  </button>
                {/if}
              </div>

              <!-- Daftar Baris Belanja -->
              <div class="flex-1 p-3 overflow-y-auto divide-y divide-slate-100">
                {#if cart.length === 0}
                  <div class="h-full flex flex-col items-center justify-center text-slate-400 text-xs">
                    <Barcode class="w-10 h-10 mb-2 stroke-1 text-slate-300" />
                    <span>Scan barcode atau klik produk untuk menambah</span>
                  </div>
                {:else}
                  {#each cart as item, idx}
                    <!-- svelte-ignore a11y_click_events_have_key_events -->
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      onclick={() => (selectedCartIndex = idx)}
                      class="transition-all cursor-pointer {selectedCartIndex === idx
                        ? 'p-2 bg-emerald-50/70 border border-emerald-400 rounded-lg shadow-2xs space-y-1.5'
                        : 'py-2.5 first:pt-0 last:pb-0 space-y-1.5 border-b border-slate-100'}"
                    >
                      <div class="flex justify-between items-start gap-2">
                        <div class="flex-1">
                          <div class="font-semibold text-slate-900 text-xs line-clamp-1 flex items-center gap-1.5">
                            {#if selectedCartIndex === idx}
                              <span class="w-1.5 h-1.5 rounded-full bg-emerald-600"></span>
                            {/if}
                            <span>{item.product.name}</span>
                          </div>
                          <div class="text-[11px] text-slate-500 font-mono">
                            @ Rp {item.unit_price.toLocaleString("id-ID")}
                          </div>
                        </div>
                        <button
                          onclick={(e) => {
                            e.stopPropagation();
                            removeFromCart(idx);
                          }}
                          class="text-slate-300 hover:text-rose-600 transition-colors p-1"
                          title="Hapus baris"
                        >
                          <Trash2 class="w-3.5 h-3.5" />
                        </button>
                      </div>

                      <div class="flex justify-between items-center">
                        <!-- Pilihan Multi-Satuan jika produk punya > 1 satuan -->
                        {#if item.product.units.length > 1}
                          <select
                            value={item.selected_unit.id}
                            onchange={(e) => changeCartUnit(idx, e.currentTarget.value)}
                            class="text-[11px] bg-slate-50 border border-slate-300 rounded px-1.5 py-0.5 font-medium text-slate-700"
                          >
                            {#each item.product.units as u}
                              <option value={u.id}>{u.unit_name} (Rp {u.retail_price.toLocaleString("id-ID")})</option>
                            {/each}
                          </select>
                        {:else}
                          <span class="text-[11px] text-slate-500">{item.selected_unit.unit_name}</span>
                        {/if}

                        <!-- Pengatur Kuantitas (+ / -) & Klik untuk Edit Qty (F4) -->
                        <div class="flex items-center gap-1">
                          <button
                            onclick={(e) => {
                              e.stopPropagation();
                              updateQuantity(idx, -1);
                            }}
                            class="w-6 h-6 rounded bg-slate-100 hover:bg-slate-200 border border-slate-300 flex items-center justify-center text-slate-700 active:scale-95"
                          >
                            <Minus class="w-3 h-3" />
                          </button>
                          <button
                            type="button"
                            onclick={(e) => {
                              e.stopPropagation();
                              selectedCartIndex = idx;
                              isEditQtyModalOpen = true;
                            }}
                            title="Klik untuk ubah kuantitas (Shortcut F4)"
                            class="w-8 py-0.5 text-center font-bold font-mono text-xs text-slate-900 bg-white hover:bg-emerald-50 hover:text-emerald-800 rounded border border-slate-200"
                          >
                            {item.quantity}
                          </button>
                          <button
                            onclick={(e) => {
                              e.stopPropagation();
                              updateQuantity(idx, 1);
                            }}
                            class="w-6 h-6 rounded bg-slate-100 hover:bg-slate-200 border border-slate-300 flex items-center justify-center text-slate-700 active:scale-95"
                          >
                            <Plus class="w-3 h-3" />
                          </button>
                          <div class="w-20 text-right font-bold font-mono text-xs text-slate-900">
                            Rp {item.subtotal.toLocaleString("id-ID")}
                          </div>
                        </div>
                      </div>
                    </div>
                  {/each}
                {/if}
              </div>

              <!-- Ringkasan Nilai & Tombol Bayar -->
              <div class="p-3 bg-slate-50 border-t border-slate-200 space-y-2">
                <!-- Rincian Pajak -->
                <div class="space-y-1 text-xs text-slate-600">
                  <div class="flex justify-between">
                    <span>Subtotal:</span>
                    <span class="font-semibold text-slate-800">Rp {cartSubtotal.toLocaleString("id-ID")}</span>
                  </div>
                  {#if taxMode === "inclusive"}
                    <div class="flex justify-between text-[11px] text-slate-500">
                      <span>Termasuk PPN 11%:</span>
                      <span>Rp {Math.round(taxTotal).toLocaleString("id-ID")}</span>
                    </div>
                  {:else if taxMode === "exclusive"}
                    <div class="flex justify-between text-[11px] text-slate-500">
                      <span>PPN 11% (Tambahan):</span>
                      <span>Rp {Math.round(taxTotal).toLocaleString("id-ID")}</span>
                    </div>
                  {/if}
                  <div class="flex justify-between items-baseline pt-1.5 border-t border-slate-200 text-slate-900">
                    <span class="font-bold text-sm">TOTAL AKHIR:</span>
                    <span class="font-bold text-lg font-mono text-emerald-700">
                      Rp {grandTotal.toLocaleString("id-ID")}
                    </span>
                  </div>
                </div>

                <!-- Tombol Tahan / Buka Nota -->
                <div class="grid grid-cols-2 gap-1.5">
                  <button
                    onclick={holdCurrentCart}
                    disabled={cart.length === 0}
                    class="py-1.5 px-2 bg-slate-200 hover:bg-slate-300 disabled:opacity-40 text-slate-800 rounded text-xs font-medium flex items-center justify-center gap-1 border border-slate-300"
                  >
                    <PauseCircle class="w-3.5 h-3.5 text-slate-600" />
                    <span>Tahan (F8)</span>
                  </button>
                  <button
                    onclick={() => (isHeldCartsModalOpen = true)}
                    disabled={heldCarts.length === 0}
                    class="py-1.5 px-2 bg-slate-200 hover:bg-slate-300 disabled:opacity-40 text-slate-800 rounded text-xs font-medium flex items-center justify-center gap-1 border border-slate-300"
                  >
                    <PlayCircle class="w-3.5 h-3.5 text-slate-600" />
                    <span>Recall ({heldCarts.length}) (F9)</span>
                  </button>
                </div>

                <!-- Tombol Pembayaran Utama -->
                <button
                  type="button"
                  disabled={cart.length === 0}
                  onclick={() => (isPaymentModalOpen = true)}
                  class="w-full py-3 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 disabled:cursor-not-allowed text-white font-bold text-sm rounded-lg shadow-xs flex items-center justify-center gap-2 transition-all active:scale-[0.99]"
                >
                  <CheckCircle class="w-5 h-5" />
                  <span>BAYAR SELESAI (F12)</span>
                </button>
              </div>
            </div>
          </div>

        <!-- TAB 2: KASIR GROSIR & PARTAI BESAR (B2B) -->
        {:else if activeTab === "wholesale"}
          <div class="p-4 flex-1 flex flex-col gap-3 overflow-hidden">
            <div class="bg-white border border-slate-200 rounded-lg p-3 flex justify-between items-center shadow-xs">
              <div>
                <h2 class="text-sm font-bold text-slate-900 flex items-center gap-2">
                  <Truck class="w-4 h-4 text-emerald-600" />
                  <span>Penjualan Partai Besar & Grosir (B2B)</span>
                </h2>
                <p class="text-xs text-slate-500">Pilih mitra toko rekanan untuk faktur tempo atau harga volume kartonan</p>
              </div>

              <!-- Pemilih Pelanggan Mitra & Tambah Baru -->
              <div class="flex items-center gap-2">
                <span class="text-xs font-semibold text-slate-700">Mitra Toko:</span>
                <select
                  bind:value={selectedWholesaleCustomerId}
                  class="text-xs bg-slate-50 border border-slate-300 rounded-lg px-3 py-1.5 font-semibold text-slate-800 focus:outline-none focus:ring-1 focus:ring-emerald-500"
                >
                  <option value={null}>-- Pilih Pelanggan B2B --</option>
                  {#each customers as c}
                    <option value={c.id}>
                      {c.name} ({c.code}) - Plafon: Rp {c.credit_limit.toLocaleString("id-ID")}
                    </option>
                  {/each}
                </select>
                <button
                  type="button"
                  onclick={() => (isAddCustomerModalOpen = true)}
                  class="px-2.5 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1 shadow-xs active:scale-95"
                >
                  <Plus class="w-3.5 h-3.5" />
                  <span>Mitra Baru</span>
                </button>
              </div>
            </div>

            <!-- Jika ada Mitra yang dipilih, tampilkan ringkasan Plafon & Piutang Berjalan -->
            {#if selectedWholesaleCustomer}
              {@const sisaPlafon = selectedWholesaleCustomer.credit_limit - selectedWholesaleCustomer.current_debt}
              <div class="bg-emerald-50/60 border border-emerald-200 rounded-lg p-3 grid grid-cols-1 md:grid-cols-4 gap-3 text-xs shadow-xs">
                <div>
                  <span class="text-slate-500 block text-[11px]">Mitra Terpilih</span>
                  <span class="font-bold text-slate-900">{selectedWholesaleCustomer.name} ({selectedWholesaleCustomer.code})</span>
                  <span class="text-[11px] text-slate-500 block">{selectedWholesaleCustomer.phone || "-"}</span>
                </div>
                <div>
                  <span class="text-slate-500 block text-[11px]">Plafon Kredit</span>
                  <span class="font-bold font-mono text-slate-800">Rp {selectedWholesaleCustomer.credit_limit.toLocaleString("id-ID")}</span>
                </div>
                <div>
                  <span class="text-slate-500 block text-[11px]">Piutang Berjalan</span>
                  <span class="font-bold font-mono {selectedWholesaleCustomer.current_debt > 0 ? 'text-rose-600' : 'text-slate-700'}">
                    Rp {selectedWholesaleCustomer.current_debt.toLocaleString("id-ID")}
                  </span>
                </div>
                <div>
                  <span class="text-slate-500 block text-[11px]">Sisa Plafon</span>
                  <span class="font-bold font-mono {sisaPlafon > 0 ? 'text-emerald-700' : 'text-rose-600'}">
                    Rp {sisaPlafon.toLocaleString("id-ID")}
                    {#if sisaPlafon <= 0}
                      <span class="text-[10px] ml-1 px-1 bg-rose-100 text-rose-700 rounded font-bold">TERLAMPAUI</span>
                    {/if}
                  </span>
                </div>
              </div>
            {/if}

            <!-- Tabel Matrix Produk & Tier Grosir -->
            <div class="flex-1 bg-white border border-slate-200 rounded-lg overflow-auto shadow-xs">
              <table class="w-full text-left text-xs border-collapse">
                <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold sticky top-0">
                  <tr>
                    <th class="py-2.5 px-3">SKU / Barcode</th>
                    <th class="py-2.5 px-3">Nama Produk</th>
                    <th class="py-2.5 px-3">Stok Gudang</th>
                    <th class="py-2.5 px-3">Satuan</th>
                    <th class="py-2.5 px-3 text-right">Harga Eceran</th>
                    <th class="py-2.5 px-3 text-right">Tier 1 (&ge;6)</th>
                    <th class="py-2.5 px-3 text-right">Tier 2 (&ge;24 / Dus)</th>
                    <th class="py-2.5 px-3 text-center">Aksi</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100 text-slate-800">
                  {#each products as prod}
                    {#each prod.units as unit}
                      <tr class="hover:bg-slate-50 transition-colors">
                        <td class="py-2.5 px-3 font-mono text-slate-500">{prod.sku}</td>
                        <td class="py-2.5 px-3 font-semibold text-slate-900">{prod.name}</td>
                        <td class="py-2.5 px-3">{prod.current_stock} {prod.base_unit_name}</td>
                        <td class="py-2.5 px-3 font-medium text-emerald-800">{unit.unit_name}</td>
                        <td class="py-2.5 px-3 text-right font-mono">Rp {unit.retail_price.toLocaleString("id-ID")}</td>
                        <td class="py-2.5 px-3 text-right font-mono text-slate-700">
                          {unit.wholesale_price_1 ? `Rp ${unit.wholesale_price_1.toLocaleString("id-ID")}` : "-"}
                        </td>
                        <td class="py-2.5 px-3 text-right font-mono font-bold text-emerald-700">
                          {unit.wholesale_price_2 ? `Rp ${unit.wholesale_price_2.toLocaleString("id-ID")}` : "-"}
                        </td>
                        <td class="py-2.5 px-3 text-center">
                          <button
                            onclick={() => addToCart(prod, unit)}
                            class="px-2.5 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded font-bold text-[11px] shadow-xs active:scale-95"
                          >
                            + Masuk Order
                          </button>
                        </td>
                      </tr>
                    {/each}
                  {/each}
                </tbody>
              </table>
            </div>

            <!-- Riwayat Order Grosir & Cetak Dokumen A4 (Faktur Pajak & Surat Jalan) -->
            {#if wholesaleRecentTransactions.length > 0}
              <div class="bg-white border border-slate-200 rounded-lg shadow-xs overflow-hidden shrink-0">
                <div class="px-4 py-2.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
                  <span class="font-bold text-xs text-slate-800 uppercase tracking-wide flex items-center gap-1.5">
                    <FileSpreadsheet class="w-3.5 h-3.5 text-emerald-600" />
                    Pesanan Grosir B2B Terakhir
                  </span>
                  <span class="text-[11px] text-slate-500">{wholesaleRecentTransactions.length} Faktur</span>
                </div>
                <div class="max-h-48 overflow-y-auto">
                  <table class="w-full text-left text-xs border-collapse">
                    <thead class="bg-slate-100 text-slate-600 border-b border-slate-200 uppercase font-semibold sticky top-0">
                      <tr>
                        <th class="py-2 px-3">No Faktur</th>
                        <th class="py-2 px-3">Pelanggan B2B</th>
                        <th class="py-2 px-3 text-right">Total Transaksi</th>
                        <th class="py-2 px-3 text-center">Status Bayar</th>
                        <th class="py-2 px-3 text-center">Jatuh Tempo</th>
                        <th class="py-2 px-3 text-center">Cetak Dokumen A4</th>
                      </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-100 text-slate-800">
                      {#each wholesaleRecentTransactions as tx}
                        <tr class="hover:bg-slate-50">
                          <td class="py-2 px-3 font-mono font-bold text-slate-900">{tx.invoice_number}</td>
                          <td class="py-2 px-3 font-semibold text-slate-800">{tx.customer_name || "Umum"}</td>
                          <td class="py-2 px-3 text-right font-mono font-bold text-emerald-700">Rp {tx.grand_total.toLocaleString("id-ID")}</td>
                          <td class="py-2 px-3 text-center">
                            <span class="px-1.5 py-0.5 rounded text-[10px] font-bold uppercase {tx.payment_status === 'paid' ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : tx.payment_status === 'partial' ? 'bg-amber-50 text-amber-700 border border-amber-200' : 'bg-rose-50 text-rose-700 border border-rose-200'}">
                              {tx.payment_status === 'paid' ? 'Lunas' : tx.payment_status === 'partial' ? 'DP / Sebagian' : 'Tempo (Hutang)'}
                            </span>
                          </td>
                          <td class="py-2 px-3 text-center font-mono text-[11px] text-slate-600">{tx.due_date || "-"}</td>
                          <td class="py-2 px-3 text-center">
                            <div class="flex items-center justify-center gap-1.5">
                              <button
                                type="button"
                                onclick={() => openWholesaleInvoice(tx.invoice_number)}
                                class="px-2 py-0.5 bg-emerald-50 hover:bg-emerald-100 text-emerald-800 border border-emerald-300 rounded text-[11px] font-semibold flex items-center gap-1 transition-colors"
                                title="Buka Faktur Penjualan & Pajak A4"
                              >
                                <Printer class="w-3 h-3 text-emerald-700" />
                                <span>Faktur A4</span>
                              </button>
                              <button
                                type="button"
                                onclick={() => openDeliveryOrder(tx.invoice_number)}
                                class="px-2 py-0.5 bg-blue-50 hover:bg-blue-100 text-blue-800 border border-blue-300 rounded text-[11px] font-semibold flex items-center gap-1 transition-colors"
                                title="Buka Surat Jalan (DO) Pengiriman"
                              >
                                <Truck class="w-3 h-3 text-blue-700" />
                                <span>Surat Jalan</span>
                              </button>
                            </div>
                          </td>
                        </tr>
                      {/each}
                    </tbody>
                  </table>
                </div>
              </div>
            {/if}
          </div>

        <!-- TAB 3: INVENTARIS & MULTI-SATUAN -->
        {:else if activeTab === "inventory"}
          <div class="p-4 flex-1 flex flex-col gap-3 overflow-hidden">
            <div class="bg-white border border-slate-200 rounded-lg p-3 flex justify-between items-center shadow-xs">
              <div>
                <h2 class="text-sm font-bold text-slate-900 flex items-center gap-2">
                  <Boxes class="w-4 h-4 text-emerald-600" />
                  <span>Katalog Produk, Multi-Satuan & Stok Opname</span>
                </h2>
                <p class="text-xs text-slate-500">Konversi satuan (Dus, Pak, Pcs), batas stok menipis, dan aturan pajak BKP</p>
              </div>
              <div class="flex items-center gap-2">
                <button
                  type="button"
                  onclick={() => (isStockOpnameModalOpen = true)}
                  class="px-3 py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-800 rounded-lg text-xs font-bold flex items-center gap-1.5 border border-slate-300"
                >
                  <Boxes class="w-4 h-4 text-slate-600" />
                  <span>Stok Opname Fisik</span>
                </button>
                <button
                  type="button"
                  onclick={() => (isAddProductModalOpen = true)}
                  class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-xs"
                >
                  <PackagePlus class="w-4 h-4" />
                  <span>Tambah Produk & Multi-Satuan</span>
                </button>
              </div>
            </div>

            <div class="flex-1 bg-white border border-slate-200 rounded-lg overflow-auto shadow-xs">
              <table class="w-full text-left text-xs border-collapse">
                <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold sticky top-0">
                  <tr>
                    <th class="py-2.5 px-3">SKU</th>
                    <th class="py-2.5 px-3">Barcode</th>
                    <th class="py-2.5 px-3">Nama Produk</th>
                    <th class="py-2.5 px-3">Kategori</th>
                    <th class="py-2.5 px-3">Pajak</th>
                    <th class="py-2.5 px-3">Sisa Stok</th>
                    <th class="py-2.5 px-3">Satuan Terdaftar</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100 text-slate-800">
                  {#each products as p}
                    <tr class="hover:bg-slate-50">
                      <td class="py-2.5 px-3 font-mono text-slate-500">{p.sku}</td>
                      <td class="py-2.5 px-3 font-mono">{p.barcode || "-"}</td>
                      <td class="py-2.5 px-3 font-bold text-slate-900">{p.name}</td>
                      <td class="py-2.5 px-3 text-slate-600">{p.category_name || "-"}</td>
                      <td class="py-2.5 px-3">
                        {#if p.is_taxable}
                          <span class="text-[10px] bg-emerald-50 text-emerald-700 border border-emerald-200 px-1.5 py-0.5 rounded font-semibold">BKP (Kena Pajak)</span>
                        {:else}
                          <span class="text-[10px] bg-slate-100 text-slate-600 border border-slate-200 px-1.5 py-0.5 rounded">Non-BKP (Bebas)</span>
                        {/if}
                      </td>
                      <td class="py-2.5 px-3 font-mono font-bold {p.current_stock <= p.min_stock_alert ? 'text-amber-600' : 'text-slate-800'}">
                        {p.current_stock} {p.base_unit_name}
                      </td>
                      <td class="py-2.5 px-3">
                        <div class="flex flex-wrap gap-1">
                          {#each p.units as u}
                            <span class="px-1.5 py-0.5 bg-slate-100 border border-slate-200 text-slate-700 rounded text-[10px]">
                              {u.unit_name} (x{u.multiplier}) = Rp {u.retail_price.toLocaleString("id-ID")}
                            </span>
                          {/each}
                        </div>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>

        <!-- TAB 4: SUPPLIER & PENGADAAN (HULU) -->
        {:else if activeTab === "procurement"}
          <div class="p-4 flex-1 flex flex-col gap-3 overflow-hidden">
            <!-- Header Toolbar & Subtabs -->
            <div class="bg-white border border-slate-200 rounded-lg p-3 flex justify-between items-center shadow-xs">
              <div class="flex items-center gap-4">
                <div>
                  <h2 class="text-sm font-bold text-slate-900 flex items-center gap-2">
                    <ReceiptText class="w-4 h-4 text-emerald-600" />
                    <span>Manajemen Pengadaan & Supplier (Hulu)</span>
                  </h2>
                  <p class="text-[11px] text-slate-500">Purchase Order (PO), Penerimaan Barang, dan Hutang Dagang</p>
                </div>

                <!-- Subtab Pills -->
                <div class="flex bg-slate-100 p-0.5 rounded-lg border border-slate-200 text-xs font-semibold">
                  <button
                    type="button"
                    onclick={() => (procurementSubTab = "po")}
                    class="px-3 py-1 rounded-md transition-colors {procurementSubTab === 'po' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
                  >
                    Purchase Order ({purchases.length})
                  </button>
                  <button
                    type="button"
                    onclick={() => (procurementSubTab = "suppliers")}
                    class="px-3 py-1 rounded-md transition-colors {procurementSubTab === 'suppliers' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
                  >
                    Daftar Supplier ({suppliers.length})
                  </button>
                  <button
                    type="button"
                    onclick={() => (procurementSubTab = "debt")}
                    class="px-3 py-1 rounded-md transition-colors {procurementSubTab === 'debt' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
                  >
                    Hutang Usaha (AP)
                  </button>
                </div>
              </div>

              <!-- Tombol Aksi Sesuai Subtab -->
              <div class="flex items-center gap-2">
                {#if procurementSubTab === "po"}
                  <button
                    type="button"
                    onclick={() => (isCreatePOModalOpen = true)}
                    class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-xs"
                  >
                    <Plus class="w-4 h-4" />
                    <span>Buat PO Baru</span>
                  </button>
                {:else if procurementSubTab === "suppliers"}
                  <button
                    type="button"
                    onclick={() => (isAddSupplierModalOpen = true)}
                    class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-xs"
                  >
                    <Plus class="w-4 h-4" />
                    <span>Tambah Supplier</span>
                  </button>
                {/if}
              </div>
            </div>

            <!-- Konten Subtab 1: Purchase Order (PO) -->
            {#if procurementSubTab === "po"}
              <div class="flex-1 bg-white border border-slate-200 rounded-lg overflow-auto shadow-xs">
                <table class="w-full text-left text-xs border-collapse">
                  <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold sticky top-0">
                    <tr>
                      <th class="py-2.5 px-3">No. PO</th>
                      <th class="py-2.5 px-3">Supplier</th>
                      <th class="py-2.5 px-3">Tipe Bayar</th>
                      <th class="py-2.5 px-3">Jatuh Tempo</th>
                      <th class="py-2.5 px-3 text-right">Subtotal</th>
                      <th class="py-2.5 px-3 text-right">Total Nilai</th>
                      <th class="py-2.5 px-3 text-center">Status</th>
                      <th class="py-2.5 px-3 text-center">Aksi Gudang</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-slate-100 text-slate-800">
                    {#if purchases.length === 0}
                      <tr>
                        <td colspan="8" class="text-center py-8 text-slate-400">Belum ada Purchase Order (PO). Klik "Buat PO Baru" untuk memesan barang.</td>
                      </tr>
                    {:else}
                      {#each purchases as po}
                        <tr class="hover:bg-slate-50 transition-colors">
                          <td class="py-2.5 px-3 font-mono font-bold text-slate-900">{po.po_number}</td>
                          <td class="py-2.5 px-3 font-semibold text-slate-800">{po.supplier_name}</td>
                          <td class="py-2.5 px-3 capitalize">
                            <span class="text-[10px] px-1.5 py-0.5 rounded font-bold {po.payment_type === 'cash' ? 'bg-emerald-50 text-emerald-700 border border-emerald-200' : 'bg-amber-50 text-amber-800 border border-amber-200'}">
                              {po.payment_type}
                            </span>
                          </td>
                          <td class="py-2.5 px-3 font-mono text-slate-600">{po.due_date || "-"}</td>
                          <td class="py-2.5 px-3 text-right font-mono">Rp {po.subtotal.toLocaleString("id-ID")}</td>
                          <td class="py-2.5 px-3 text-right font-mono font-bold text-slate-900">
                            Rp {po.total_amount.toLocaleString("id-ID")}
                          </td>
                          <td class="py-2.5 px-3 text-center">
                            {#if po.status === "received"}
                              <span class="text-[10px] bg-emerald-100 text-emerald-800 font-bold px-2 py-0.5 rounded-full">
                                Diterima & Masuk Stok
                              </span>
                            {:else}
                              <span class="text-[10px] bg-blue-100 text-blue-800 font-bold px-2 py-0.5 rounded-full">
                                Dipesan (Menunggu Barang)
                              </span>
                            {/if}
                          </td>
                          <td class="py-2.5 px-3 text-center">
                            {#if po.status === "ordered"}
                              <button
                                type="button"
                                onclick={() => handleReceivePurchase(po.id)}
                                class="px-2.5 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded font-bold text-[11px] shadow-xs active:scale-95 flex items-center gap-1 mx-auto"
                              >
                                <CheckCircle class="w-3.5 h-3.5" />
                                <span>Terima Barang</span>
                              </button>
                            {:else}
                              <span class="text-slate-400 text-[11px]">Selesai</span>
                            {/if}
                          </td>
                        </tr>
                      {/each}
                    {/if}
                  </tbody>
                </table>
              </div>

            <!-- Konten Subtab 2: Daftar Supplier -->
            {:else if procurementSubTab === "suppliers"}
              <div class="grid grid-cols-1 md:grid-cols-2 gap-3 overflow-y-auto">
                {#each suppliers as sup}
                  <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs space-y-2">
                    <div class="flex justify-between items-start">
                      <div>
                        <h3 class="font-bold text-slate-900 text-sm">{sup.name}</h3>
                        <p class="text-xs text-slate-500">{sup.company_name || "-"}</p>
                      </div>
                      <span class="text-xs bg-slate-100 text-slate-700 px-2 py-0.5 rounded border border-slate-200 font-medium">
                        Tempo: {sup.payment_terms_days} Hari
                      </span>
                    </div>
                    <div class="text-xs text-slate-600 space-y-1 pt-2 border-t border-slate-100">
                      <div>Telepon: <span class="font-mono text-slate-900">{sup.phone || "-"}</span></div>
                      <div>NPWP: <span class="font-mono text-slate-700">{sup.npwp || "-"}</span></div>
                      <div>Alamat: <span>{sup.address || "-"}</span></div>
                    </div>
                  </div>
                {/each}
              </div>

            <!-- Konten Subtab 3: Hutang Usaha (Accounts Payable) -->
            {:else if procurementSubTab === "debt"}
              <div class="flex-1 bg-white border border-slate-200 rounded-lg overflow-auto shadow-xs">
                <table class="w-full text-left text-xs border-collapse">
                  <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold sticky top-0">
                    <tr>
                      <th class="py-2.5 px-3">No. Faktur / PO</th>
                      <th class="py-2.5 px-3">Pemasok</th>
                      <th class="py-2.5 px-3">Jatuh Tempo</th>
                      <th class="py-2.5 px-3 text-right">Total Tagihan</th>
                      <th class="py-2.5 px-3 text-right">Sudah Dibayar</th>
                      <th class="py-2.5 px-3 text-right">Sisa Hutang</th>
                      <th class="py-2.5 px-3 text-center">Aksi Bayar</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-slate-100 text-slate-800">
                    {#if unpaidPurchases.length === 0}
                      <tr>
                        <td colspan="7" class="text-center py-8 text-slate-400">Tidak ada tagihan hutang supplier yang belum lunas. Semua lunas!</td>
                      </tr>
                    {:else}
                      {#each unpaidPurchases as p}
                        {@const sisa = p.total_amount - p.paid_amount}
                        <tr class="hover:bg-slate-50">
                          <td class="py-2.5 px-3 font-mono font-bold text-slate-900">{p.po_number}</td>
                          <td class="py-2.5 px-3 font-semibold text-slate-800">{p.supplier_name}</td>
                          <td class="py-2.5 px-3 font-mono text-amber-700 font-bold">{p.due_date || "-"}</td>
                          <td class="py-2.5 px-3 text-right font-mono">Rp {p.total_amount.toLocaleString("id-ID")}</td>
                          <td class="py-2.5 px-3 text-right font-mono text-emerald-700">Rp {p.paid_amount.toLocaleString("id-ID")}</td>
                          <td class="py-2.5 px-3 text-right font-mono font-bold text-rose-600">
                            Rp {sisa.toLocaleString("id-ID")}
                          </td>
                          <td class="py-2.5 px-3 text-center">
                            <button
                              type="button"
                              onclick={() => handlePayPurchaseDebt(p.id, sisa)}
                              class="px-2.5 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded font-bold text-[11px] shadow-xs active:scale-95"
                            >
                              Lunasi Hutang
                            </button>
                          </td>
                        </tr>
                      {/each}
                    {/if}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>

        <!-- TAB 5: PELANGGAN & PIUTANG -->
        {:else if activeTab === "customers"}
          <div class="p-4 flex-1 flex flex-col gap-3 overflow-hidden">
            <!-- Header Toolbar & Subtabs -->
            <div class="bg-white border border-slate-200 rounded-lg p-3 flex justify-between items-center shadow-xs">
              <div class="flex items-center gap-4">
                <div>
                  <h2 class="text-sm font-bold text-slate-900 flex items-center gap-2">
                    <Users class="w-4 h-4 text-emerald-600" />
                    <span>Database Pelanggan & Plafon Piutang (Kredit Limit)</span>
                  </h2>
                  <p class="text-[11px] text-slate-500">Pelacakan batas kredit pelanggan grosir, jatuh tempo, dan pelunasan piutang</p>
                </div>

                <!-- Subtab Pills -->
                <div class="flex bg-slate-100 p-0.5 rounded-lg border border-slate-200 text-xs font-semibold">
                  <button
                    type="button"
                    onclick={() => (customerSubTab = "list")}
                    class="px-3 py-1 rounded-md transition-colors {customerSubTab === 'list' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
                  >
                    Semua Pelanggan ({customers.length})
                  </button>
                  <button
                    type="button"
                    onclick={() => (customerSubTab = "receivables")}
                    class="px-3 py-1 rounded-md transition-colors {customerSubTab === 'receivables' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
                  >
                    Piutang Tempo Berjalan ({unpaidCustomerList.length})
                  </button>
                </div>
              </div>

              <!-- Tombol Aksi Tambah Pelanggan -->
              <button
                type="button"
                onclick={() => (isAddCustomerModalOpen = true)}
                class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-xs active:scale-95"
              >
                <Plus class="w-4 h-4" />
                <span>Tambah Pelanggan Baru</span>
              </button>
            </div>

            <!-- Konten Subtab 1: Grid Kartu Semua Pelanggan -->
            {#if customerSubTab === "list"}
              <div class="flex-1 overflow-y-auto">
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
                  {#each customers as c}
                    {@const debtRatio = c.credit_limit > 0 ? Math.min(100, Math.round((c.current_debt / c.credit_limit) * 100)) : 0}
                    {@const sisaPlafon = c.credit_limit - c.current_debt}
                    <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs space-y-3 flex flex-col justify-between">
                      <div>
                        <div class="flex justify-between items-start">
                          <div>
                            <div class="flex items-center gap-1.5">
                              <h3 class="font-bold text-slate-900 text-sm">{c.name}</h3>
                              <span class="text-[10px] uppercase font-bold px-1.5 py-0.2 rounded {c.customer_type === 'wholesale' ? 'bg-blue-50 text-blue-700 border border-blue-200' : c.customer_type === 'vip' ? 'bg-amber-50 text-amber-700 border border-amber-200' : 'bg-slate-100 text-slate-600'}">
                                {c.customer_type}
                              </span>
                            </div>
                            <p class="text-xs text-slate-500 font-mono">Kode: {c.code}</p>
                          </div>
                        </div>

                        <div class="text-xs text-slate-600 space-y-1 pt-2.5 border-t border-slate-100">
                          <div class="flex justify-between">
                            <span class="text-slate-500">Telepon:</span>
                            <span class="font-mono text-slate-800">{c.phone || "-"}</span>
                          </div>
                          {#if c.npwp_nik}
                            <div class="flex justify-between">
                              <span class="text-slate-500">NPWP / NIK:</span>
                              <span class="font-mono text-slate-800">{c.npwp_nik}</span>
                            </div>
                          {/if}
                          {#if c.address}
                            <div class="text-[11px] text-slate-500 truncate" title={c.address}>
                              Alamat: {c.address}
                            </div>
                          {/if}

                          <!-- Plafon & Pemakaian -->
                          <div class="pt-2 border-t border-slate-100 space-y-1">
                            <div class="flex justify-between items-baseline text-xs">
                              <span class="text-slate-500">Plafon Kredit:</span>
                              <span class="font-bold font-mono text-slate-800">Rp {c.credit_limit.toLocaleString("id-ID")}</span>
                            </div>
                            <div class="flex justify-between items-baseline text-xs">
                              <span class="text-slate-500">Piutang Berjalan:</span>
                              <span class="font-bold font-mono {c.current_debt > 0 ? 'text-rose-600' : 'text-emerald-700'}">
                                Rp {c.current_debt.toLocaleString("id-ID")}
                              </span>
                            </div>
                            <div class="flex justify-between items-baseline text-xs">
                              <span class="text-slate-500">Sisa Plafon:</span>
                              <span class="font-bold font-mono {sisaPlafon > 0 ? 'text-emerald-700' : 'text-rose-600'}">
                                Rp {sisaPlafon.toLocaleString("id-ID")}
                              </span>
                            </div>

                            <!-- Progress Bar Plafon -->
                            <div class="w-full bg-slate-100 rounded-full h-1.5 overflow-hidden mt-1.5">
                              <div
                                class="h-full rounded-full transition-all {debtRatio >= 90 ? 'bg-rose-500' : debtRatio >= 60 ? 'bg-amber-500' : 'bg-emerald-500'}"
                                style="width: {debtRatio}%"
                              ></div>
                            </div>
                          </div>
                        </div>
                      </div>

                      <!-- Tombol Pelunasan jika ada piutang -->
                      <div class="pt-2 border-t border-slate-100 flex justify-between items-center">
                        <span class="text-[11px] text-slate-500">
                          {c.current_debt > 0 ? `Terpakai: ${debtRatio}%` : 'Lunas / Nihil'}
                        </span>
                        {#if c.current_debt > 0}
                          <button
                            type="button"
                            onclick={() => handleOpenDebtModal(c)}
                            class="px-2.5 py-1 bg-emerald-50 hover:bg-emerald-100 text-emerald-800 border border-emerald-300 rounded text-xs font-semibold shadow-2xs active:scale-95 transition-colors"
                          >
                            Catat Pelunasan
                          </button>
                        {:else}
                          <span class="text-[11px] text-emerald-700 font-medium">Bebas Piutang</span>
                        {/if}
                      </div>
                    </div>
                  {/each}
                </div>
              </div>

            <!-- Konten Subtab 2: Tabel Piutang Tempo Berjalan -->
            {:else if customerSubTab === "receivables"}
              <div class="flex-1 bg-white border border-slate-200 rounded-lg overflow-auto shadow-xs">
                <table class="w-full text-left text-xs border-collapse">
                  <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold sticky top-0">
                    <tr>
                      <th class="py-2.5 px-3">Kode</th>
                      <th class="py-2.5 px-3">Nama Mitra Pelanggan</th>
                      <th class="py-2.5 px-3">No Telepon</th>
                      <th class="py-2.5 px-3">NPWP / NIK</th>
                      <th class="py-2.5 px-3 text-right">Plafon Kredit</th>
                      <th class="py-2.5 px-3 text-right">Saldo Piutang Berjalan</th>
                      <th class="py-2.5 px-3 text-right">Sisa Plafon</th>
                      <th class="py-2.5 px-3 text-center">Aksi Pelunasan</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-slate-100 text-slate-800">
                    {#if unpaidCustomerList.length === 0}
                      <tr>
                        <td colspan="8" class="text-center py-12 text-slate-400">
                          Tidak ada piutang tempo yang berjalan. Semua tagihan mitra toko tertib lunas!
                        </td>
                      </tr>
                    {:else}
                      {#each unpaidCustomerList as c}
                        {@const sisa = c.credit_limit - c.current_debt}
                        <tr class="hover:bg-slate-50">
                          <td class="py-2.5 px-3 font-mono font-bold text-slate-900">{c.code}</td>
                          <td class="py-2.5 px-3 font-semibold text-slate-800">{c.name}</td>
                          <td class="py-2.5 px-3 font-mono text-slate-600">{c.phone || "-"}</td>
                          <td class="py-2.5 px-3 font-mono text-slate-500">{c.npwp_nik || "-"}</td>
                          <td class="py-2.5 px-3 text-right font-mono">Rp {c.credit_limit.toLocaleString("id-ID")}</td>
                          <td class="py-2.5 px-3 text-right font-mono font-bold text-rose-600">
                            Rp {c.current_debt.toLocaleString("id-ID")}
                          </td>
                          <td class="py-2.5 px-3 text-right font-mono font-bold {sisa > 0 ? 'text-emerald-700' : 'text-rose-700'}">
                            Rp {sisa.toLocaleString("id-ID")}
                          </td>
                          <td class="py-2.5 px-3 text-center">
                            <button
                              type="button"
                              onclick={() => handleOpenDebtModal(c)}
                              class="px-2.5 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded font-bold text-[11px] shadow-xs active:scale-95"
                            >
                              Catat Pelunasan
                            </button>
                          </td>
                        </tr>
                      {/each}
                    {/if}
                  </tbody>
                </table>
              </div>
            {/if}
          </div>

        <!-- TAB 6: LAPORAN & PERPAJAKAN -->
        {:else if activeTab === "reports"}
          <div class="p-4 flex-1 flex flex-col gap-4 overflow-y-auto">
            <!-- 4 Kotak KPI Ringkasan -->
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
              <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs">
                <span class="text-xs text-slate-500 font-medium">Omset Penjualan Hari Ini</span>
                <div class="text-xl font-bold font-mono text-emerald-700 mt-1">
                  Rp {(dashboardStats?.today_sales || 0).toLocaleString("id-ID")}
                </div>
                <span class="text-[11px] text-slate-400 mt-0.5 block">{dashboardStats?.today_transactions_count || 0} Transaksi</span>
              </div>
              <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs">
                <span class="text-xs text-slate-500 font-medium">Total Produk Aktif</span>
                <div class="text-xl font-bold font-mono text-slate-900 mt-1">
                  {dashboardStats?.total_products_count || 0} Item
                </div>
                <span class="text-[11px] text-slate-400 mt-0.5 block">Katalog master</span>
              </div>
              <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs">
                <span class="text-xs text-slate-500 font-medium">Peringatan Stok Menipis</span>
                <div class="text-xl font-bold font-mono text-amber-600 mt-1">
                  {dashboardStats?.low_stock_count || 0} Produk
                </div>
                <span class="text-[11px] text-slate-400 mt-0.5 block">&le; Batas Minimum</span>
              </div>
              <div class="bg-white border border-slate-200 rounded-lg p-4 shadow-xs">
                <span class="text-xs text-slate-500 font-medium">Status Rekap Pajak (PPN)</span>
                <div class="text-xl font-bold font-mono text-blue-700 mt-1">
                  11%
                </div>
                <span class="text-[11px] text-slate-400 mt-0.5 block">Mode: {taxMode.toUpperCase()}</span>
              </div>
            </div>

            <!-- Tabel Transaksi Terakhir -->
            <div class="bg-white border border-slate-200 rounded-lg shadow-xs overflow-hidden">
              <div class="px-4 py-3 bg-slate-50 border-b border-slate-200 font-bold text-xs text-slate-800 uppercase tracking-wide">
                20 Riwayat Transaksi Kasir Terakhir
              </div>
              <table class="w-full text-left text-xs border-collapse">
                <thead class="bg-slate-100 text-slate-600 border-b border-slate-200 uppercase font-semibold">
                  <tr>
                    <th class="py-2.5 px-3">No Invoice</th>
                    <th class="py-2.5 px-3">Tipe</th>
                    <th class="py-2.5 px-3">Kasir</th>
                    <th class="py-2.5 px-3">Pelanggan</th>
                    <th class="py-2.5 px-3 text-right">Subtotal</th>
                    <th class="py-2.5 px-3 text-right">PPN</th>
                    <th class="py-2.5 px-3 text-right">Total Akhir</th>
                    <th class="py-2.5 px-3 text-center">Waktu</th>
                    <th class="py-2.5 px-3 text-center">Aksi</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-slate-100 text-slate-800">
                  {#if recentTransactions.length === 0}
                    <tr>
                      <td colspan="9" class="text-center py-8 text-slate-400">Belum ada riwayat transaksi</td>
                    </tr>
                  {:else}
                    {#each recentTransactions as tx}
                      <tr class="hover:bg-slate-50">
                        <td class="py-2.5 px-3 font-mono font-bold text-slate-900">{tx.invoice_number}</td>
                        <td class="py-2.5 px-3">
                          <span class="text-[10px] uppercase font-bold px-1.5 py-0.2 rounded {tx.order_type === 'wholesale' ? 'bg-blue-50 text-blue-700' : 'bg-emerald-50 text-emerald-700'}">
                            {tx.order_type}
                          </span>
                        </td>
                        <td class="py-2.5 px-3 text-slate-700">{tx.cashier_name || tx.cashier_id}</td>
                        <td class="py-2.5 px-3 text-slate-600">{tx.customer_name || "Umum"}</td>
                        <td class="py-2.5 px-3 text-right font-mono">Rp {tx.subtotal.toLocaleString("id-ID")}</td>
                        <td class="py-2.5 px-3 text-right font-mono text-slate-600">Rp {tx.tax_total.toLocaleString("id-ID")}</td>
                        <td class="py-2.5 px-3 text-right font-mono font-bold text-emerald-700">
                          Rp {tx.grand_total.toLocaleString("id-ID")}
                        </td>
                        <td class="py-2.5 px-3 text-center text-slate-500 font-mono text-[11px]">
                          {new Date(tx.created_at).toLocaleTimeString("id-ID")}
                        </td>
                        <td class="py-2.5 px-3 text-center">
                          <button
                            type="button"
                            onclick={() => openReceiptForTransaction(tx.invoice_number)}
                            class="px-2.5 py-1 bg-emerald-50 hover:bg-emerald-100 text-emerald-800 border border-emerald-300 rounded text-[11px] font-semibold flex items-center gap-1 mx-auto transition-colors"
                            title="Buka & Cetak Struk Thermal"
                          >
                            <Printer class="w-3 h-3 text-emerald-700" />
                            <span>Struk</span>
                          </button>
                        </td>
                      </tr>
                    {/each}
                  {/if}
                </tbody>
              </table>
            </div>
          </div>

        <!-- TAB 7: PENGATURAN TOKO & JARINGAN -->
        {:else if activeTab === "settings"}
          <div class="p-4 flex-1 flex flex-col gap-4 overflow-y-auto max-w-3xl">
            <div class="bg-white border border-slate-200 rounded-lg p-5 shadow-xs space-y-4">
              <h2 class="text-sm font-bold text-slate-900 border-b border-slate-200 pb-2 flex items-center gap-2">
                <Building2 class="w-4 h-4 text-emerald-600" />
                <span>Kustomisasi Profil Toko & Cabang</span>
              </h2>

              <div class="grid grid-cols-2 gap-3 text-xs">
                <div>
                  <label for="settings-store-name" class="block text-slate-700 font-semibold mb-1">Nama Toko</label>
                  <input
                    id="settings-store-name"
                    type="text"
                    value={storeName}
                    class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900"
                  />
                </div>
                <div>
                  <label for="settings-store-branch" class="block text-slate-700 font-semibold mb-1">Nama Cabang</label>
                  <input
                    id="settings-store-branch"
                    type="text"
                    value={storeBranch}
                    class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900"
                  />
                </div>
              </div>

              <!-- Konfigurasi Pajak Default -->
              <div class="pt-3 border-t border-slate-200">
                <h3 class="text-xs font-bold text-slate-900 mb-2 flex items-center gap-1.5">
                  <Percent class="w-3.5 h-3.5 text-emerald-600" />
                  <span>Mode Perhitungan Pajak Default</span>
                </h3>
                <div class="flex gap-3 text-xs">
                  <label class="flex items-center gap-2">
                    <input
                      type="radio"
                      name="tax_mode"
                      value="inclusive"
                      checked={taxMode === "inclusive"}
                      class="text-emerald-600"
                    />
                    <span>Tax Inclusive (Harga sudah termasuk PPN 11%)</span>
                  </label>
                  <label class="flex items-center gap-2">
                    <input
                      type="radio"
                      name="tax_mode"
                      value="exclusive"
                      checked={taxMode === "exclusive"}
                      class="text-emerald-600"
                    />
                    <span>Tax Exclusive (PPN 11% ditambahkan di akhir nota)</span>
                  </label>
                </div>
              </div>

              <!-- Konfigurasi Mode Jaringan Server Lokal (Sprint 2) -->
              <div class="pt-3 border-t border-slate-200 space-y-3">
                <div class="flex justify-between items-center">
                  <h3 class="text-xs font-bold text-slate-900 flex items-center gap-1.5">
                    <Wifi class="w-3.5 h-3.5 text-emerald-600" />
                    <span>Embedded Server Kasir Lokal (LAN Host)</span>
                  </h3>
                  <div class="flex items-center gap-2">
                    <button
                      type="button"
                      onclick={handleTestPing}
                      class="px-2.5 py-1 text-xs bg-slate-100 hover:bg-slate-200 text-slate-700 font-medium rounded border border-slate-300"
                    >
                      Tes Ping
                    </button>
                    <button
                      type="button"
                      onclick={handleToggleServer}
                      class="px-3 py-1 text-xs font-bold rounded text-white {serverInfo?.is_running ? 'bg-rose-600 hover:bg-rose-700' : 'bg-emerald-600 hover:bg-emerald-700'}"
                    >
                      {serverInfo?.is_running ? 'Hentikan Server' : 'Aktifkan Server'}
                    </button>
                  </div>
                </div>

                <!-- Status Server Realtime Card -->
                <div class="p-3.5 bg-slate-50 border border-slate-200 rounded-lg space-y-2.5 text-xs">
                  <div class="flex justify-between items-center">
                    <div class="flex items-center gap-2">
                      <span class="w-2.5 h-2.5 rounded-full {serverInfo?.is_running ? 'bg-emerald-500 animate-pulse' : 'bg-slate-400'}"></span>
                      <span class="font-bold text-slate-900">
                        {serverInfo?.is_running ? 'Server Aktif (Siap Menerima Klien LAN)' : 'Server Non-aktif'}
                      </span>
                    </div>
                    <span class="font-mono text-xs bg-white px-2 py-0.5 rounded border border-slate-200 text-slate-700">
                      Port: {serverInfo?.port || serverPortInput}
                    </span>
                  </div>

                  {#if serverInfo?.is_running}
                    <div class="grid grid-cols-2 gap-2 pt-2 border-t border-slate-200 text-[11px]">
                      <div>
                        <span class="text-slate-500">Alamat IP Host Lokal:</span>
                        <div class="font-mono font-bold text-emerald-700 text-xs mt-0.5">
                          http://{serverInfo.host_ip}:{serverInfo.port}
                        </div>
                      </div>
                      <div>
                        <span class="text-slate-500">Kasir Klien Terhubung:</span>
                        <div class="font-bold text-slate-900 text-xs mt-0.5">
                          {serverInfo.connected_clients} Komputer Klien (WebSocket Aktif)
                        </div>
                      </div>
                    </div>
                  {/if}
                </div>

                <!-- Pilihan Mode Peran Komputer -->
                <div class="space-y-2 text-xs">
                  <div class="font-semibold text-slate-700">Peran Komputer Ini:</div>
                  <div class="grid grid-cols-3 gap-2">
                    <button
                      type="button"
                      onclick={() => (networkRole = "server")}
                      class="p-2.5 rounded-lg border text-left transition-colors {networkRole === 'server' ? 'border-emerald-600 bg-emerald-50 text-emerald-950 ring-1 ring-emerald-600' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'}"
                    >
                      <div class="font-bold">Kasir Utama (Host)</div>
                      <div class="text-[10px] text-slate-500 mt-0.5">Menyimpan SQLite master & melayani klien</div>
                    </button>

                    <button
                      type="button"
                      onclick={() => (networkRole = "client")}
                      class="p-2.5 rounded-lg border text-left transition-colors {networkRole === 'client' ? 'border-emerald-600 bg-emerald-50 text-emerald-950 ring-1 ring-emerald-600' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'}"
                    >
                      <div class="font-bold">Kasir Klien (Worker)</div>
                      <div class="text-[10px] text-slate-500 mt-0.5">Terhubung ke IP Kasir Utama via Wi-Fi/LAN</div>
                    </button>

                    <button
                      type="button"
                      onclick={() => (networkRole = "standalone")}
                      class="p-2.5 rounded-lg border text-left transition-colors {networkRole === 'standalone' ? 'border-emerald-600 bg-emerald-50 text-emerald-950 ring-1 ring-emerald-600' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'}"
                    >
                      <div class="font-bold">Mandiri (Standalone)</div>
                      <div class="text-[10px] text-slate-500 mt-0.5">Berjalan sendiri tanpa jaringan LAN</div>
                    </button>
                  </div>

                  {#if networkRole === "client"}
                    <div class="mt-2 p-3 bg-blue-50 border border-blue-200 rounded-lg space-y-2">
                      <label for="remote-host-ip" class="block text-xs font-semibold text-blue-900">
                        Alamat IP & Port Kasir Utama:
                      </label>
                      <div class="flex gap-2">
                        <input
                          id="remote-host-ip"
                          type="text"
                          bind:value={clientRemoteHost}
                          placeholder="misal: 192.168.1.100:8080"
                          class="flex-1 px-3 py-1.5 bg-white border border-blue-300 rounded font-mono text-xs text-slate-900"
                        />
                        <button
                          type="button"
                          onclick={() => showToast(`Mencoba menyambungkan ke ${clientRemoteHost}...`)}
                          class="px-3 py-1.5 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded text-xs"
                        >
                          Hubungkan
                        </button>
                      </div>
                    </div>
                  {/if}
                </div>
              </div>
            </div>
          </div>
        {/if}
      {/if}
    </main>
  </div>

  <!-- MODAL PEMBAYARAN -->
  <PaymentModal
    isOpen={isPaymentModalOpen}
    {grandTotal}
    subtotal={cartSubtotal}
    {taxBase}
    {taxTotal}
    {taxMode}
    {paymentMethods}
    orderType={activeTab === "wholesale" ? "wholesale" : "retail"}
    customer={selectedWholesaleCustomer}
    onClose={() => (isPaymentModalOpen = false)}
    onConfirmPayment={handleConfirmPayment}
  />

  <!-- MODAL MANAJEMEN SHIFT KASIR -->
  <ShiftModal
    isOpen={isShiftModalOpen}
    {currentShift}
    onClose={() => (isShiftModalOpen = false)}
    onStartShift={handleStartShift}
    onCloseShift={handleCloseShift}
    onAddCashMovement={handleAddCashMovement}
  />

  <!-- MODAL TAMBAH PRODUK & MULTI-SATUAN (SPRINT 3) -->
  <AddProductModal
    isOpen={isAddProductModalOpen}
    {categories}
    onClose={() => (isAddProductModalOpen = false)}
    onSubmit={handleCreateProductFull}
  />

  <!-- MODAL TERBITKAN PURCHASE ORDER (SPRINT 3) -->
  <CreatePOModal
    isOpen={isCreatePOModalOpen}
    {suppliers}
    {products}
    onClose={() => (isCreatePOModalOpen = false)}
    onSubmit={handleCreatePurchase}
  />

  <!-- MODAL TAMBAH SUPPLIER BARU (SPRINT 3) -->
  <AddSupplierModal
    isOpen={isAddSupplierModalOpen}
    onClose={() => (isAddSupplierModalOpen = false)}
    onSubmit={handleCreateSupplier}
  />

  <!-- MODAL STOK OPNAME FISIK (SPRINT 3) -->
  <StockOpnameModal
    isOpen={isStockOpnameModalOpen}
    {products}
    onClose={() => (isStockOpnameModalOpen = false)}
    onSubmit={handleAdjustStock}
  />

  <!-- MODAL STRUK THERMAL 58MM / 80MM (SPRINT 4) -->
  <ReceiptModal
    isOpen={isReceiptModalOpen}
    transaction={activeReceiptDetail}
    onClose={() => (isReceiptModalOpen = false)}
  />

  <!-- MODAL TRANSAKSI TERTAHAN / HELD CARTS (SPRINT 4) -->
  <HeldCartsModal
    isOpen={isHeldCartsModalOpen}
    {heldCarts}
    onClose={() => (isHeldCartsModalOpen = false)}
    onRecall={handleRecallHeldCart}
    onDelete={handleDeleteHeldCart}
  />

  <!-- MODAL UBAH KUANTITAS BARIS AKTIF (F4) (SPRINT 4) -->
  <EditQtyModal
    isOpen={isEditQtyModalOpen}
    item={cart[selectedCartIndex] || null}
    onClose={() => (isEditQtyModalOpen = false)}
    onSaveQty={handleSaveEditedQty}
  />

  <!-- MODAL TAMBAH PELANGGAN / MITRA GROSIR (SPRINT 5) -->
  <AddCustomerModal
    isOpen={isAddCustomerModalOpen}
    onClose={() => (isAddCustomerModalOpen = false)}
    onSubmit={handleCreateCustomer}
  />

  <!-- MODAL BUKU PEMBANTU PIUTANG & CATAT PELUNASAN (SPRINT 5) -->
  <CustomerDebtModal
    isOpen={isCustomerDebtModalOpen}
    customer={selectedDebtCustomer}
    {paymentMethods}
    onClose={() => (isCustomerDebtModalOpen = false)}
    onPaymentSuccess={handleCustomerPaymentSuccess}
  />

  <!-- MODAL FAKTUR PENJUALAN & PAJAK A4 (SPRINT 5) -->
  <WholesaleInvoiceModal
    isOpen={isWholesaleInvoiceModalOpen}
    transaction={activeReceiptDetail}
    onClose={() => (isWholesaleInvoiceModalOpen = false)}
  />

  <!-- MODAL SURAT JALAN / DELIVERY ORDER A4 (SPRINT 5) -->
  <DeliveryOrderModal
    isOpen={isDeliveryOrderModalOpen}
    transaction={activeReceiptDetail}
    onClose={() => (isDeliveryOrderModalOpen = false)}
  />
</div>
