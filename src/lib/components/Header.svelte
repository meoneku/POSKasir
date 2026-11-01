<script lang="ts">
  import { Store, User, Clock, ShieldCheck, Lock, LogOut, KeyRound } from "@lucide/svelte";
  import type { Shift, ServerInfo, UserSession } from "$lib/types";

  interface Props {
    storeName?: string;
    branchName?: string;
    shift: Shift | null;
    currentUser: UserSession | null;
    serverInfo?: ServerInfo | null;
    onOpenShiftModal: () => void;
    onLockScreen: () => void;
    onLogout: () => void;
  }

  let {
    storeName = "BERKAH POS KASIR",
    branchName = "Kantor Pusat",
    shift = null,
    currentUser = null,
    serverInfo = null,
    onOpenShiftModal,
    onLockScreen,
    onLogout,
  }: Props = $props();

  let timeString = $state("");

  $effect(() => {
    const updateTime = () => {
      const now = new Date();
      timeString = now.toLocaleTimeString("id-ID", {
        hour: "2-digit",
        minute: "2-digit",
        second: "2-digit",
      });
    };
    updateTime();
    const timer = setInterval(updateTime, 1000);
    return () => clearInterval(timer);
  });
</script>

<header class="h-14 bg-white border-b border-slate-200 px-4 flex items-center justify-between select-none shrink-0 z-20">
  <!-- Sisi Kiri: Identitas Toko & Status Server -->
  <div class="flex items-center gap-3">
    <div class="w-8 h-8 rounded bg-emerald-600 text-white flex items-center justify-center font-bold text-sm tracking-wider shadow-xs">
      <Store class="w-4 h-4" />
    </div>
    <div>
      <div class="flex items-center gap-2">
        <h1 class="font-bold text-slate-900 leading-tight text-sm tracking-tight">{storeName}</h1>
        <span class="text-xs bg-slate-100 text-slate-600 px-1.5 py-0.5 rounded border border-slate-200">
          {branchName}
        </span>
      </div>
      <div class="flex items-center gap-2 text-[11px] text-slate-500">
        {#if serverInfo?.is_running}
          <div class="flex items-center gap-1.5 font-mono">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse inline-block"></span>
            <span class="text-emerald-700 font-semibold">{serverInfo.host_ip}:{serverInfo.port}</span>
          </div>
          {#if serverInfo.connected_clients > 0}
            <span class="text-[10px] bg-emerald-100 text-emerald-800 font-bold px-1.5 py-0.2 rounded border border-emerald-300">
              {serverInfo.connected_clients} Klien
            </span>
          {/if}
        {:else}
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-slate-400 inline-block"></span>
            <span>Server Lokal Non-aktif</span>
          </div>
        {/if}
      </div>
    </div>
  </div>

  <!-- Sisi Kanan: Status Shift, Jam, Profil Kasir & Tombol Kunci -->
  <div class="flex items-center gap-2.5">
    <!-- Tombol Shift Kasir -->
    <button
      onclick={onOpenShiftModal}
      class="flex items-center gap-2 px-3 py-1.5 rounded text-xs font-medium border transition-colors {shift?.status === 'open' ? 'bg-emerald-50 text-emerald-800 border-emerald-300 hover:bg-emerald-100' : 'bg-amber-50 text-amber-800 border-amber-300 hover:bg-amber-100'}"
    >
      <ShieldCheck class="w-3.5 h-3.5 {shift?.status === 'open' ? 'text-emerald-600' : 'text-amber-600'}" />
      <span>{shift?.status === "open" ? `Shift Kasir Aktif` : "Buka Shift Kasir"}</span>
      {#if shift?.status === "open"}
        <span class="font-semibold text-emerald-950 ml-1">
          Rp {shift.starting_cash.toLocaleString("id-ID")}
        </span>
      {/if}
    </button>

    <!-- Waktu & Jam -->
    <div class="hidden sm:flex items-center gap-1.5 text-xs text-slate-600 bg-slate-100 px-2.5 py-1.5 rounded border border-slate-200">
      <Clock class="w-3.5 h-3.5 text-slate-500" />
      <span class="font-mono font-medium">{timeString || "00:00:00"}</span>
    </div>

    <!-- Tombol Kunci Layar (Lock Screen) -->
    <button
      type="button"
      onclick={onLockScreen}
      title="Kunci Layar Kasir Sementara (Membutuhkan PIN untuk membuka)"
      class="p-2 rounded-lg text-slate-500 hover:text-slate-800 hover:bg-slate-100 border border-slate-200 transition-colors"
    >
      <Lock class="w-3.5 h-3.5" />
    </button>

    <!-- Profil Pengguna Aktif (Klik untuk kelola shift / profil) -->
    <button
      type="button"
      onclick={onOpenShiftModal}
      title="Profil pengguna bertugas. Klik untuk melihat shift."
      class="flex items-center gap-2 pl-2 border-l border-slate-200 hover:opacity-85 transition-opacity text-left cursor-pointer group"
    >
      <div class="w-7 h-7 rounded-full {currentUser?.role === 'admin' ? 'bg-amber-100 text-amber-800 border-amber-300' : 'bg-emerald-100 text-emerald-800 border-emerald-300'} border flex items-center justify-center font-bold text-xs">
        {(currentUser?.full_name || shift?.cashier_name || "K").slice(0, 1).toUpperCase()}
      </div>
      <div class="text-left hidden md:block">
        <div class="text-xs font-semibold text-slate-900 group-hover:text-emerald-800 leading-tight">
          {currentUser?.full_name || shift?.cashier_name || "Kasir Utama"}
        </div>
        <div class="text-[10px] text-slate-500 capitalize flex items-center gap-1">
          <span>{currentUser?.role === 'admin' ? 'Superadmin' : currentUser?.role === 'supervisor' ? 'Supervisor' : 'Kasir'}</span>
        </div>
      </div>
    </button>

    <!-- Tombol Logout -->
    <button
      type="button"
      onclick={onLogout}
      title="Keluar dari akun (Logout)"
      class="p-2 rounded-lg text-rose-500 hover:text-rose-700 hover:bg-rose-50 border border-slate-200 hover:border-rose-300 transition-colors"
    >
      <LogOut class="w-3.5 h-3.5" />
    </button>
  </div>
</header>
