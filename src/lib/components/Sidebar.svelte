<script lang="ts">
  import {
    ShoppingCart,
    Truck,
    Boxes,
    Network,
    ReceiptText,
    Users,
    BarChart3,
    Settings,
  } from "@lucide/svelte";

  interface Props {
    activeTab: string;
    userRole?: string;
    onChangeTab: (tab: string) => void;
  }

  let { activeTab = "pos", userRole = "admin", onChangeTab }: Props = $props();

  const allMenuItems = [
    { id: "pos", label: "Kasir Retail", icon: ShoppingCart, shortcut: "F1", roles: ["cashier", "supervisor", "admin"] },
    { id: "wholesale", label: "Kasir Grosir", icon: Truck, shortcut: "", roles: ["cashier", "supervisor", "admin"] },
    { id: "inventory", label: "Inventaris & Stok", icon: Boxes, shortcut: "", roles: ["cashier", "supervisor", "admin"] },
    { id: "branches", label: "Multi-Cabang & Mutasi", icon: Network, shortcut: "", roles: ["cashier", "supervisor", "admin"] },
    { id: "customers", label: "Pelanggan & Piutang", icon: Users, shortcut: "", roles: ["cashier", "supervisor", "admin"] },
    { id: "procurement", label: "Supplier & PO", icon: ReceiptText, shortcut: "", roles: ["supervisor", "admin"] },
    { id: "reports", label: "Laporan & Pajak", icon: BarChart3, shortcut: "", roles: ["supervisor", "admin"] },
    { id: "settings", label: "Pengaturan Toko", icon: Settings, shortcut: "", roles: ["admin"] },
  ];

  let menuItems = $derived(
    allMenuItems.filter((item) => item.roles.includes(userRole))
  );
</script>

<aside class="w-56 bg-slate-900 text-slate-300 border-r border-slate-800 flex flex-col justify-between shrink-0 select-none">
  <div class="p-3 space-y-1">
    <div class="px-3 py-2 text-[10px] font-bold uppercase tracking-wider text-slate-400">
      Navigasi Sistem
    </div>

    {#each menuItems as item}
      {@const Icon = item.icon}
      {@const isActive = activeTab === item.id}
      <button
        onclick={() => onChangeTab(item.id)}
        class="w-full flex items-center justify-between px-3 py-2.5 rounded text-xs font-medium transition-colors {isActive
          ? 'bg-emerald-600 text-white font-semibold'
          : 'text-slate-300 hover:bg-slate-800 hover:text-white'}"
      >
        <div class="flex items-center gap-2.5">
          <Icon class="w-4 h-4 {isActive ? 'text-white' : 'text-slate-400'}" />
          <span>{item.label}</span>
        </div>
        {#if item.shortcut}
          <span class="text-[10px] px-1 py-0.2 rounded bg-slate-800 text-slate-400 border border-slate-700 font-mono">
            {item.shortcut}
          </span>
        {/if}
      </button>
    {/each}
  </div>

  <!-- Bantuan Tombol Pintas Keyboard -->
  <div class="p-3 m-3 bg-slate-800/80 rounded border border-slate-700/60 text-[11px] text-slate-400 space-y-1.5">
    <div class="font-semibold text-slate-200 text-xs mb-1">Shortcut Kasir</div>
    <div class="flex justify-between items-center">
      <span>Cari / Barcode</span>
      <kbd class="px-1.5 py-0.5 bg-slate-900 rounded border border-slate-700 text-slate-300 font-mono text-[10px]">F2</kbd>
    </div>
    <div class="flex justify-between items-center">
      <span>Ganti Kuantitas</span>
      <kbd class="px-1.5 py-0.5 bg-slate-900 rounded border border-slate-700 text-slate-300 font-mono text-[10px]">F4</kbd>
    </div>
    <div class="flex justify-between items-center">
      <span>Tahan Nota</span>
      <kbd class="px-1.5 py-0.5 bg-slate-900 rounded border border-slate-700 text-slate-300 font-mono text-[10px]">F8</kbd>
    </div>
    <div class="flex justify-between items-center">
      <span>Buka Nota Tahan</span>
      <kbd class="px-1.5 py-0.5 bg-slate-900 rounded border border-slate-700 text-slate-300 font-mono text-[10px]">F9</kbd>
    </div>
    <div class="flex justify-between items-center">
      <span>Bayar & Selesai</span>
      <kbd class="px-1.5 py-0.5 bg-slate-900 rounded border border-slate-700 text-emerald-400 font-mono text-[10px]">F12</kbd>
    </div>
  </div>
</aside>
