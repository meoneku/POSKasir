<script lang="ts">
  import { X, CheckCircle2, DollarSign, QrCode, CreditCard, Building } from "@lucide/svelte";
  import type { PaymentMethod, Tax } from "$lib/types";

  interface Props {
    isOpen: boolean;
    grandTotal: number;
    subtotal: number;
    taxBase: number;
    taxTotal: number;
    taxMode: "inclusive" | "exclusive" | "none";
    paymentMethods: PaymentMethod[];
    onClose: () => void;
    onConfirmPayment: (methodId: string, amountPaid: number, changeDue: number) => void;
  }

  let {
    isOpen = false,
    grandTotal = 0,
    subtotal = 0,
    taxBase = 0,
    taxTotal = 0,
    taxMode = "inclusive",
    paymentMethods = [],
    onClose,
    onConfirmPayment,
  }: Props = $props();

  let selectedMethodId = $state("");
  let amountPaid = $state(0);
  let inputRef = $state<HTMLInputElement | null>(null);

  // Inisialisasi metode pembayaran default ke Tunai (cash)
  $effect(() => {
    if (isOpen && paymentMethods.length > 0) {
      const cashMethod = paymentMethods.find((m) => m.category === "cash") || paymentMethods[0];
      selectedMethodId = cashMethod.id;
      amountPaid = grandTotal;

      // Smart autofocus dan seleksi teks
      setTimeout(() => {
        if (inputRef) {
          inputRef.focus();
          inputRef.select();
        }
      }, 50);
    }
  });

  let changeDue = $derived(Math.max(0, amountPaid - grandTotal));
  let isSufficient = $derived(amountPaid >= grandTotal);

  const quickNominals = [10000, 20000, 50000, 100000, 200000];

  function setExact() {
    amountPaid = grandTotal;
    inputRef?.focus();
  }

  function addNominal(val: number) {
    amountPaid = val;
    inputRef?.focus();
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "Enter" && isSufficient) {
      e.preventDefault();
      handleSubmit();
    }
  }

  function handleSubmit() {
    if (!isSufficient) return;
    onConfirmPayment(selectedMethodId, amountPaid, changeDue);
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={handleKeydown}
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col">
      <!-- Header Dialog -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div>
          <h3 class="font-bold text-slate-900 text-base">Pembayaran & Selesaikan Transaksi</h3>
          <p class="text-xs text-slate-500">Pilih metode bayar dan masukkan nominal uang</p>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <!-- Ringkasan Tagihan & Pajak -->
        <div class="bg-slate-50 border border-slate-200 rounded-lg p-3.5 space-y-1.5 text-xs text-slate-600">
          <div class="flex justify-between">
            <span>Subtotal Belanja:</span>
            <span class="font-semibold text-slate-800">Rp {subtotal.toLocaleString("id-ID")}</span>
          </div>

          {#if taxMode === "inclusive"}
            <div class="flex justify-between text-[11px] text-slate-500 pt-1 border-t border-slate-200">
              <span>Dasar Pengenaan Pajak (DPP):</span>
              <span>Rp {Math.round(taxBase).toLocaleString("id-ID")}</span>
            </div>
            <div class="flex justify-between text-[11px] text-slate-500">
              <span>PPN Terhitung (Termasuk):</span>
              <span>Rp {Math.round(taxTotal).toLocaleString("id-ID")}</span>
            </div>
          {:else if taxMode === "exclusive"}
            <div class="flex justify-between text-[11px] text-slate-500 pt-1 border-t border-slate-200">
              <span>PPN Tambahan (11%):</span>
              <span>Rp {Math.round(taxTotal).toLocaleString("id-ID")}</span>
            </div>
          {/if}

          <div class="flex justify-between items-baseline pt-2 border-t border-slate-300 text-slate-900">
            <span class="font-bold text-sm">TOTAL HARUS DIBAYAR:</span>
            <span class="font-bold text-xl text-emerald-700 font-mono">
              Rp {grandTotal.toLocaleString("id-ID")}
            </span>
          </div>
        </div>

        <!-- Pemilihan Metode Pembayaran -->
        <div>
          <div class="block text-xs font-semibold text-slate-700 mb-1.5">
            Metode Pembayaran
          </div>
          <div class="grid grid-cols-3 gap-2">
            {#each paymentMethods as pm}
              <button
                type="button"
                onclick={() => (selectedMethodId = pm.id)}
                class="px-3 py-2 rounded-lg border text-xs font-medium text-left flex items-center gap-2 transition-colors {selectedMethodId === pm.id
                  ? 'border-emerald-600 bg-emerald-50 text-emerald-900 font-semibold ring-1 ring-emerald-600'
                  : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50'}"
              >
                {#if pm.category === "cash"}
                  <DollarSign class="w-4 h-4 text-emerald-600 shrink-0" />
                {:else if pm.category === "ewallet"}
                  <QrCode class="w-4 h-4 text-blue-600 shrink-0" />
                {:else if pm.category === "bank_transfer"}
                  <Building class="w-4 h-4 text-slate-600 shrink-0" />
                {:else}
                  <CreditCard class="w-4 h-4 text-slate-600 shrink-0" />
                {/if}
                <span class="truncate">{pm.name}</span>
              </button>
            {/each}
          </div>
        </div>

        <!-- Input Nominal Uang Diterima -->
        <div>
          <label for="payment-cash-input" class="block text-xs font-semibold text-slate-700 mb-1">
            Nominal Uang Diterima (Rp)
          </label>
          <div class="relative">
            <span class="absolute left-3.5 top-1/2 -translate-y-1/2 font-bold text-slate-400 text-sm">Rp</span>
            <input
              id="payment-cash-input"
              bind:this={inputRef}
              type="number"
              bind:value={amountPaid}
              class="w-full pl-11 pr-4 py-2.5 bg-white border border-slate-300 rounded-lg text-lg font-mono font-bold text-slate-900 focus:outline-none focus:border-emerald-600 focus:ring-1 focus:ring-emerald-600"
            />
          </div>

          <!-- Tombol Nominal Cepat -->
          <div class="flex flex-wrap gap-1.5 mt-2">
            <button
              type="button"
              onclick={setExact}
              class="px-2.5 py-1 text-xs font-medium bg-slate-100 hover:bg-slate-200 text-slate-800 rounded border border-slate-300"
            >
              Uang Pas
            </button>
            {#each quickNominals as nom}
              <button
                type="button"
                onclick={() => addNominal(nom)}
                class="px-2.5 py-1 text-xs font-medium bg-slate-100 hover:bg-slate-200 text-slate-800 rounded border border-slate-300 font-mono"
              >
                {nom.toLocaleString("id-ID")}
              </button>
            {/each}
          </div>
        </div>

        <!-- Tampilan Kembalian -->
        <div class="bg-emerald-50 border border-emerald-200 rounded-lg p-3 flex justify-between items-center">
          <span class="text-xs font-semibold text-emerald-900">UANG KEMBALIAN:</span>
          <span class="text-xl font-bold font-mono text-emerald-700">
            Rp {changeDue.toLocaleString("id-ID")}
          </span>
        </div>
      </div>

      <!-- Footer Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-t border-slate-200 flex justify-between items-center">
        <span class="text-[11px] text-slate-500 font-mono">
          [ENTER] Konfirmasi &middot; [ESC] Batal
        </span>
        <div class="flex gap-2">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 text-xs font-medium text-slate-700 bg-white border border-slate-300 hover:bg-slate-50 rounded-lg transition-colors"
          >
            Batal (ESC)
          </button>
          <button
            type="button"
            disabled={!isSufficient}
            onclick={handleSubmit}
            class="px-5 py-2 text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed rounded-lg shadow-xs flex items-center gap-1.5 transition-colors"
          >
            <CheckCircle2 class="w-4 h-4" />
            <span>Selesaikan & Cetak Struk (F12)</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
