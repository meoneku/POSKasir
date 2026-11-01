<script lang="ts">
  import { X, ShieldCheck, ArrowDownCircle, ArrowUpCircle, Check, FileText, Printer, Calculator } from "@lucide/svelte";
  import type { Shift, ShiftReport } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    currentShift: Shift | null;
    onClose: () => void;
    onStartShift: (startingCash: number) => void;
    onCloseShift: (actualCash: number, notes?: string) => void;
    onAddCashMovement: (type: "in" | "out", amount: number, reason: string) => void;
  }

  let {
    isOpen = false,
    currentShift = null,
    onClose,
    onStartShift,
    onCloseShift,
    onAddCashMovement,
  }: Props = $props();

  let activeTab = $state<"status" | "movement" | "report" | "close">("status");
  let startingCashInput = $state(500000);
  let actualCashInput = $state(0);
  let closeNotes = $state("");

  let movementType = $state<"in" | "out">("out");
  let movementAmount = $state(50000);
  let movementReason = $state("");

  let shiftReport = $state<ShiftReport | null>(null);
  let isReportLoading = $state(false);

  $effect(() => {
    if (isOpen && currentShift?.id && activeTab === "report") {
      loadReport(currentShift.id);
    }
  });

  async function loadReport(shiftId: string) {
    isReportLoading = true;
    try {
      shiftReport = await api.getShiftReport(shiftId);
    } catch (err) {
      console.error("Gagal memuat shift report:", err);
    } finally {
      isReportLoading = false;
    }
  }

  function handleStartShift() {
    onStartShift(startingCashInput);
  }

  function handleCloseShift() {
    onCloseShift(actualCashInput, closeNotes);
  }

  function handleSaveMovement() {
    if (movementAmount <= 0 || !movementReason.trim()) return;
    onAddCashMovement(movementType, movementAmount, movementReason);
    movementReason = "";
    activeTab = "status";
  }

  function handlePrintReport() {
    window.print();
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4 print:p-0 print:bg-white print:fixed print:inset-0"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-md overflow-hidden flex flex-col print:border-none print:shadow-none print:max-w-none print:w-auto">
      <!-- Header Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center print:hidden">
        <div class="flex items-center gap-2">
          <ShieldCheck class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">Manajemen Shift & Kas Laci</h3>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Tab Navigasi Shift jika Shift Sedang Buka -->
      {#if currentShift?.status === "open"}
        <div class="flex border-b border-slate-200 bg-slate-100 text-xs print:hidden">
          <button
            onclick={() => (activeTab = "status")}
            class="flex-1 py-2 font-medium border-b-2 {activeTab === 'status' ? 'border-emerald-600 text-emerald-800 bg-white font-semibold' : 'border-transparent text-slate-600 hover:text-slate-900'}"
          >
            Ringkasan
          </button>
          <button
            onclick={() => (activeTab = "movement")}
            class="flex-1 py-2 font-medium border-b-2 {activeTab === 'movement' ? 'border-emerald-600 text-emerald-800 bg-white font-semibold' : 'border-transparent text-slate-600 hover:text-slate-900'}"
          >
            Kas +/-
          </button>
          <button
            onclick={() => {
              activeTab = "report";
              if (currentShift?.id) loadReport(currentShift.id);
            }}
            class="flex-1 py-2 font-medium border-b-2 {activeTab === 'report' ? 'border-emerald-600 text-emerald-800 bg-white font-semibold' : 'border-transparent text-slate-600 hover:text-slate-900'}"
          >
            X-Report
          </button>
          <button
            onclick={() => (activeTab = "close")}
            class="flex-1 py-2 font-medium border-b-2 {activeTab === 'close' ? 'border-emerald-600 text-emerald-800 bg-white font-semibold' : 'border-transparent text-slate-600 hover:text-slate-900'}"
          >
            Tutup Shift
          </button>
        </div>
      {/if}

      <div class="p-5 max-h-[80vh] overflow-y-auto print:p-0">
        {#if !currentShift || currentShift.status === "closed"}
          <!-- Form Buka Shift Baru -->
          <div class="space-y-4">
            <div class="p-3 bg-amber-50 border border-amber-200 rounded-lg text-xs text-amber-900">
              Shift kasir saat ini sedang tutup. Silakan masukkan modal awal uang receh di laci kasir untuk mulai transaksi.
            </div>

            <div>
              <label for="shift-starting-cash-input" class="block text-xs font-semibold text-slate-700 mb-1">
                Modal Awal di Laci (Rp)
              </label>
              <input
                id="shift-starting-cash-input"
                type="number"
                bind:value={startingCashInput}
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-base font-mono font-bold text-slate-900 focus:outline-none focus:border-emerald-600"
              />
            </div>

            <button
              type="button"
              onclick={handleStartShift}
              class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-lg transition-colors flex items-center justify-center gap-1.5"
            >
              <Check class="w-4 h-4" />
              <span>Buka Shift & Aktifkan Kasir</span>
            </button>
          </div>
        {:else if activeTab === "status"}
          <!-- Ringkasan Shift Aktif -->
          <div class="space-y-3 text-xs text-slate-700">
            <div class="bg-slate-50 border border-slate-200 rounded-lg p-3 space-y-2">
              <div class="flex justify-between">
                <span class="text-slate-500">Kasir Bertugas:</span>
                <span class="font-bold text-slate-900">{currentShift.cashier_name || "Kasir Utama"}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-slate-500">Mulai Shift:</span>
                <span class="font-mono text-slate-800">
                  {new Date(currentShift.start_time).toLocaleTimeString("id-ID")}
                </span>
              </div>
              <div class="flex justify-between pt-2 border-t border-slate-200">
                <span class="text-slate-500">Modal Kas Awal:</span>
                <span class="font-bold font-mono text-emerald-800">
                  Rp {currentShift.starting_cash.toLocaleString("id-ID")}
                </span>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-2">
              <button
                type="button"
                onclick={() => (activeTab = "movement")}
                class="py-2 px-3 bg-slate-100 hover:bg-slate-200 text-slate-800 rounded-lg font-medium border border-slate-300 flex items-center justify-center gap-1.5"
              >
                <Calculator class="w-4 h-4 text-slate-600" />
                <span>Kas Masuk/Keluar</span>
              </button>
              <button
                type="button"
                onclick={() => {
                  activeTab = "report";
                  if (currentShift?.id) loadReport(currentShift.id);
                }}
                class="py-2 px-3 bg-slate-100 hover:bg-slate-200 text-slate-800 rounded-lg font-medium border border-slate-300 flex items-center justify-center gap-1.5"
              >
                <FileText class="w-4 h-4 text-emerald-600" />
                <span>Lihat X-Report</span>
              </button>
            </div>

            <div class="pt-2">
              <button
                type="button"
                onclick={() => (activeTab = "close")}
                class="w-full py-2.5 bg-amber-600 hover:bg-amber-700 text-white rounded-lg font-bold flex items-center justify-center gap-1.5 shadow-xs"
              >
                <span>Tutup Shift Ini (Blind Close)</span>
              </button>
            </div>
          </div>
        {:else if activeTab === "movement"}
          <!-- Form Kas Masuk / Keluar -->
          <div class="space-y-3 text-xs">
            <div class="flex gap-2">
              <button
                type="button"
                onclick={() => (movementType = "in")}
                class="flex-1 py-1.5 rounded border text-xs font-semibold flex items-center justify-center gap-1.5 {movementType === 'in' ? 'bg-emerald-50 border-emerald-600 text-emerald-800 ring-1 ring-emerald-600' : 'bg-white border-slate-300 text-slate-700'}"
              >
                <ArrowDownCircle class="w-3.5 h-3.5 text-emerald-600" />
                <span>Kas Masuk (+)</span>
              </button>
              <button
                type="button"
                onclick={() => (movementType = "out")}
                class="flex-1 py-1.5 rounded border text-xs font-semibold flex items-center justify-center gap-1.5 {movementType === 'out' ? 'bg-rose-50 border-rose-600 text-rose-800 ring-1 ring-rose-600' : 'bg-white border-slate-300 text-slate-700'}"
              >
                <ArrowUpCircle class="w-3.5 h-3.5 text-rose-600" />
                <span>Kas Keluar (-)</span>
              </button>
            </div>

            <div>
              <label for="shift-nominal-input" class="block text-slate-700 font-semibold mb-1">Nominal (Rp)</label>
              <input
                id="shift-nominal-input"
                type="number"
                bind:value={movementAmount}
                class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono font-bold"
              />
            </div>

            <div>
              <label for="shift-alasan-input" class="block text-slate-700 font-semibold mb-1">Alasan / Keperluan</label>
              <input
                id="shift-alasan-input"
                type="text"
                placeholder="misal: Ambil uang receh, Beli kertas struk"
                bind:value={movementReason}
                class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg"
              />
            </div>

            <button
              type="button"
              onclick={handleSaveMovement}
              class="w-full py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold rounded-lg"
            >
              Simpan Mutasi Kas
            </button>
          </div>
        {:else if activeTab === "report"}
          <!-- Tampilan Laporan Shift X-Report -->
          <div class="space-y-3 text-xs">
            {#if isReportLoading}
              <div class="py-8 text-center text-slate-500 animate-pulse">Menghitung rekapitulasi shift...</div>
            {:else if shiftReport}
              <div class="bg-slate-50 border border-slate-300 p-3 rounded-lg font-mono text-[11px] leading-relaxed text-slate-900 whitespace-pre overflow-x-auto shadow-inner">
                {shiftReport.report_text}
              </div>

              <div class="flex justify-end gap-2 pt-2 border-t border-slate-200 print:hidden">
                <button
                  type="button"
                  onclick={() => (activeTab = "status")}
                  class="px-3 py-1.5 bg-white hover:bg-slate-100 border border-slate-300 rounded-lg font-medium text-slate-700"
                >
                  Kembali
                </button>
                <button
                  type="button"
                  onclick={handlePrintReport}
                  class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white font-bold rounded-lg flex items-center gap-1.5 shadow-xs"
                >
                  <Printer class="w-4 h-4" />
                  <span>Cetak X-Report</span>
                </button>
              </div>
            {/if}
          </div>
        {:else if activeTab === "close"}
          <!-- Form Tutup Shift (Blind Close) -->
          <div class="space-y-3 text-xs">
            <div class="p-2.5 bg-amber-50 border border-amber-200 rounded text-amber-900 leading-relaxed">
              <strong>Metode Blind Close:</strong> Hitung fisik uang di laci terlebih dahulu dan ketikkan nominalnya di bawah ini. Sistem akan menghitung selisih kas secara otomatis dan menerbitkan Z-Report.
            </div>

            <div>
              <label for="shift-actual-cash-input" class="block text-slate-700 font-semibold mb-1">Total Uang Fisik di Laci (Rp)</label>
              <input
                id="shift-actual-cash-input"
                type="number"
                bind:value={actualCashInput}
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-base font-mono font-bold"
              />
            </div>

            <div>
              <label for="shift-notes-input" class="block text-slate-700 font-semibold mb-1">Catatan Shift (Opsional)</label>
              <textarea
                id="shift-notes-input"
                rows="2"
                placeholder="Catatan kendala atau serah terima..."
                bind:value={closeNotes}
                class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg"
              ></textarea>
            </div>

            <button
              type="button"
              onclick={handleCloseShift}
              class="w-full py-2 bg-rose-600 hover:bg-rose-700 text-white font-bold rounded-lg"
            >
              Konfirmasi Tutup Shift Kasir (Z-Report)
            </button>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  @media print {
    @page {
      size: 58mm auto;
      margin: 0;
    }
  }
</style>
