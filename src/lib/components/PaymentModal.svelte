<script lang="ts">
  import { X, CheckCircle2, DollarSign, QrCode, CreditCard, Building, Calendar, AlertTriangle, ShieldCheck } from "@lucide/svelte";
  import type { PaymentMethod, Customer } from "$lib/types";

  interface Props {
    isOpen: boolean;
    grandTotal: number;
    subtotal: number;
    taxBase: number;
    taxTotal: number;
    taxMode: "inclusive" | "exclusive" | "none";
    paymentMethods: PaymentMethod[];
    orderType?: "retail" | "wholesale";
    customer?: Customer | null;
    onClose: () => void;
    onConfirmPayment: (methodId: string, amountPaid: number, changeDue: number, dueDate?: string) => void;
  }

  let {
    isOpen = false,
    grandTotal = 0,
    subtotal = 0,
    taxBase = 0,
    taxTotal = 0,
    taxMode = "inclusive",
    paymentMethods = [],
    orderType = "retail",
    customer = null,
    onClose,
    onConfirmPayment,
  }: Props = $props();

  let paymentTerm = $state<"immediate" | "tempo">("immediate");
  let selectedMethodId = $state("");
  let amountPaid = $state(0);
  let tempoDays = $state<number>(14);
  let customDueDate = $state("");
  let supervisorApproved = $state(false);
  let inputRef = $state<HTMLInputElement | null>(null);

  // Inisialisasi metode pembayaran default
  $effect(() => {
    if (isOpen && paymentMethods.length > 0) {
      const cashMethod = paymentMethods.find((m) => m.category === "cash") || paymentMethods[0];
      selectedMethodId = cashMethod.id;
      amountPaid = grandTotal;
      paymentTerm = "immediate";
      tempoDays = 14;
      supervisorApproved = false;

      // Hitung tanggal jatuh tempo default 14 hari kedepan
      const d = new Date();
      d.setDate(d.getDate() + 14);
      customDueDate = d.toISOString().split("T")[0];

      setTimeout(() => {
        if (inputRef) {
          inputRef.focus();
          inputRef.select();
        }
      }, 50);
    }
  });

  // Hitungan Jatuh Tempo
  let computedDueDate = $derived(() => {
    if (customDueDate) return customDueDate;
    const d = new Date();
    d.setDate(d.getDate() + tempoDays);
    return d.toISOString().split("T")[0];
  });

  // Hitungan Plafon Kredit untuk Pelanggan Grosir
  let unpaidAmount = $derived(Math.max(0, grandTotal - amountPaid));
  let remainingCreditLimit = $derived(
    customer ? Math.max(0, customer.credit_limit - customer.current_debt) : 0
  );
  let isExceedingCreditLimit = $derived(
    customer && customer.credit_limit > 0 && customer.current_debt + unpaidAmount > customer.credit_limit
  );

  let changeDue = $derived(paymentTerm === "immediate" ? Math.max(0, amountPaid - grandTotal) : 0);
  let isSufficient = $derived(
    paymentTerm === "immediate"
      ? amountPaid >= grandTotal
      : !isExceedingCreditLimit || supervisorApproved
  );

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
    const due = paymentTerm === "tempo" ? computedDueDate() : undefined;
    onConfirmPayment(selectedMethodId, amountPaid, changeDue, due);
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
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col max-h-[95vh]">
      <!-- Header Dialog -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div>
          <h3 class="font-bold text-slate-900 text-base">Pembayaran & Selesaikan Transaksi</h3>
          <p class="text-xs text-slate-500">
            {orderType === "wholesale" && customer
              ? `Penjualan Grosir B2B &middot; ${customer.name}`
              : "Pilih metode bayar dan masukkan nominal uang"}
          </p>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-3.5 overflow-y-auto flex-1">
        <!-- Opsi Syarat Pembayaran (Jika Grosir / Ada Pelanggan) -->
        {#if customer || orderType === "wholesale"}
          <div class="flex bg-slate-100 p-1 rounded-lg text-xs font-semibold">
            <button
              type="button"
              onclick={() => {
                paymentTerm = "immediate";
                amountPaid = grandTotal;
              }}
              class="flex-1 py-1.5 rounded transition-colors {paymentTerm === 'immediate'
                ? 'bg-white text-emerald-800 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'}"
            >
              Pembayaran Lunas (Cash/Transfer)
            </button>
            <button
              type="button"
              onclick={() => {
                paymentTerm = "tempo";
                amountPaid = 0;
              }}
              class="flex-1 py-1.5 rounded transition-colors {paymentTerm === 'tempo'
                ? 'bg-white text-emerald-800 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'}"
            >
              Piutang Tempo (Kredit B2B)
            </button>
          </div>
        {/if}

        <!-- Ringkasan Tagihan & Pajak -->
        <div class="bg-slate-50 border border-slate-200 rounded-lg p-3.5 space-y-1 text-xs text-slate-600">
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
            <span class="font-bold text-sm">TOTAL TAGIHAN:</span>
            <span class="font-bold text-xl text-emerald-700 font-mono">
              Rp {grandTotal.toLocaleString("id-ID")}
            </span>
          </div>
        </div>

        {#if paymentTerm === "tempo" && customer}
          <!-- Bagian Khusus Piutang Tempo B2B -->
          <div class="bg-amber-50 border border-amber-200 rounded-lg p-3 space-y-2 text-xs">
            <div class="flex justify-between items-center text-amber-900 font-semibold">
              <span>Status Plafon Kredit Toko:</span>
              <span class="font-mono">Plafon: Rp {customer.credit_limit.toLocaleString("id-ID")}</span>
            </div>
            <div class="flex justify-between text-[11px] text-amber-800">
              <span>Piutang Berjalan: Rp {customer.current_debt.toLocaleString("id-ID")}</span>
              <span>Sisa Plafon: Rp {remainingCreditLimit.toLocaleString("id-ID")}</span>
            </div>

            <!-- Peringatan Melebihi Plafon Kredit -->
            {#if isExceedingCreditLimit}
              <div class="p-2 bg-rose-100 border border-rose-300 rounded text-rose-900 space-y-1">
                <div class="flex items-center gap-1.5 font-bold">
                  <AlertTriangle class="w-4 h-4 text-rose-600" />
                  <span>TRANSAKSI MELEBIHI PLAFON KREDIT!</span>
                </div>
                <p class="text-[11px]">
                  Tagihan ini akan melampaui batas plafon sebesar Rp {(customer.current_debt + unpaidAmount - customer.credit_limit).toLocaleString("id-ID")}.
                </p>
                <label class="flex items-center gap-1.5 pt-1 text-[11px] font-semibold text-rose-950 cursor-pointer">
                  <input
                    type="checkbox"
                    bind:checked={supervisorApproved}
                    class="rounded text-rose-600 focus:ring-rose-500"
                  />
                  <span>Otorisasi / Persetujuan Supervisor Diberikan</span>
                </label>
              </div>
            {/if}

            <!-- Pengaturan Jatuh Tempo -->
            <div class="pt-2 border-t border-amber-200">
              <label for="tempo-due-input" class="block font-semibold text-amber-950 mb-1">
                Pilih Tanggal Jatuh Tempo
              </label>
              <div class="grid grid-cols-4 gap-1.5 mb-2">
                {#each [7, 14, 30, 45] as days}
                  <button
                    type="button"
                    onclick={() => {
                      tempoDays = days;
                      const d = new Date();
                      d.setDate(d.getDate() + days);
                      customDueDate = d.toISOString().split("T")[0];
                    }}
                    class="py-1 rounded border text-xs font-semibold {tempoDays === days
                      ? 'bg-amber-600 text-white border-amber-600'
                      : 'bg-white text-slate-700 border-amber-200 hover:bg-amber-100'}"
                  >
                    {days} Hari
                  </button>
                {/each}
              </div>
              <input
                id="tempo-due-input"
                type="date"
                bind:value={customDueDate}
                class="w-full px-3 py-1.5 bg-white border border-amber-300 rounded-lg font-mono font-bold text-slate-900"
              />
            </div>
          </div>
        {/if}

        <!-- Pemilihan Metode Pembayaran / Uang Muka DP -->
        <div>
          <div class="flex justify-between items-center mb-1.5 text-xs font-semibold text-slate-700">
            <span>{paymentTerm === 'tempo' ? 'Metode Penyetoran Uang Muka (DP)' : 'Metode Pembayaran'}</span>
            {#if paymentTerm === 'tempo'}
              <span class="text-slate-400 font-normal">(Bisa Rp 0 jika full tempo)</span>
            {/if}
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

        <!-- Input Nominal Uang Diterima / Uang Muka (DP) -->
        <div>
          <div class="flex justify-between items-center mb-1 text-xs font-semibold text-slate-700">
            <label for="payment-cash-input">
              {paymentTerm === 'tempo' ? 'Nominal Uang Muka / DP Dibayar (Rp)' : 'Nominal Uang Diterima (Rp)'}
            </label>
            {#if paymentTerm === 'tempo'}
              <button
                type="button"
                onclick={() => (amountPaid = 0)}
                class="text-xs text-amber-700 hover:underline font-bold"
              >
                Tanpa DP (Rp 0)
              </button>
            {/if}
          </div>
          <div class="relative">
            <span class="absolute left-3.5 top-1/2 -translate-y-1/2 font-bold text-slate-400 text-sm">Rp</span>
            <input
              id="payment-cash-input"
              bind:this={inputRef}
              type="number"
              min="0"
              bind:value={amountPaid}
              class="w-full pl-11 pr-4 py-2.5 bg-white border border-slate-300 rounded-lg text-lg font-mono font-bold text-slate-900 focus:outline-none focus:border-emerald-600 focus:ring-1 focus:ring-emerald-600"
            />
          </div>

          <!-- Tombol Nominal Cepat (Hanya mode immediate) -->
          {#if paymentTerm === "immediate"}
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
          {/if}
        </div>

        <!-- Tampilan Kembalian / Sisa Piutang Tempo -->
        {#if paymentTerm === "immediate"}
          <div class="bg-emerald-50 border border-emerald-200 rounded-lg p-3 flex justify-between items-center">
            <span class="text-xs font-semibold text-emerald-900">UANG KEMBALIAN:</span>
            <span class="text-xl font-bold font-mono text-emerald-700">
              Rp {changeDue.toLocaleString("id-ID")}
            </span>
          </div>
        {:else}
          <div class="bg-amber-50 border border-amber-200 rounded-lg p-3 flex justify-between items-center">
            <span class="text-xs font-semibold text-amber-900">SISA PIUTANG TEMPO DICATAT:</span>
            <span class="text-xl font-bold font-mono text-rose-600">
              Rp {unpaidAmount.toLocaleString("id-ID")}
            </span>
          </div>
        {/if}
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
            <span>{paymentTerm === 'tempo' ? 'Proses Faktur Tempo' : 'Selesaikan Pembayaran'}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
