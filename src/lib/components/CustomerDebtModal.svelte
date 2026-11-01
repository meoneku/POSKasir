<script lang="ts">
  import { X, Receipt, Check, History, CreditCard, AlertCircle, ArrowDownLeft } from "@lucide/svelte";
  import type { Customer, CustomerDebtPayment, PaymentMethod } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    customer: Customer | null;
    paymentMethods: PaymentMethod[];
    onClose: () => void;
    onPaymentSuccess: () => void;
  }

  let {
    isOpen = false,
    customer = null,
    paymentMethods = [],
    onClose,
    onPaymentSuccess,
  }: Props = $props();

  let activeSubTab = $state<"pay" | "history">("pay");
  let payAmount = $state(0);
  let selectedMethodId = $state("");
  let referenceNumber = $state("");
  let notes = $state("");
  let historyList = $state<CustomerDebtPayment[]>([]);
  let isLoadingHistory = $state(false);

  $effect(() => {
    if (isOpen && customer) {
      payAmount = customer.current_debt;
      const bankOrCash = paymentMethods.find((m) => m.category === "bank_transfer") || paymentMethods[0];
      selectedMethodId = bankOrCash?.id || "";
      referenceNumber = "";
      notes = "";
      activeSubTab = "pay";
      loadHistory(customer.id);
    }
  });

  async function loadHistory(custId: string) {
    isLoadingHistory = true;
    try {
      historyList = await api.getCustomerDebtHistory(custId);
    } catch (err) {
      console.error("Gagal memuat histori piutang:", err);
    } finally {
      isLoadingHistory = false;
    }
  }

  async function handleSubmitPayment() {
    if (!customer || payAmount <= 0) return;
    try {
      await api.payCustomerDebt(
        customer.id,
        null,
        payAmount,
        selectedMethodId,
        referenceNumber.trim() || undefined,
        notes.trim() || undefined
      );
      onPaymentSuccess();
    } catch (err) {
      alert(`Gagal mencatat pelunasan piutang: ${err}`);
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

{#if isOpen && customer}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={handleKeydown}
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh]">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Receipt class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Pelunasan Piutang Mitra Toko</h3>
            <p class="text-[11px] text-slate-500">{customer.name} ({customer.code})</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Ringkasan Saldo & Plafon -->
      <div class="bg-slate-50 px-5 py-3 border-b border-slate-200 grid grid-cols-2 gap-3 text-xs">
        <div>
          <span class="text-slate-500">Total Piutang Berjalan:</span>
          <div class="text-base font-bold font-mono text-rose-600 mt-0.5">
            Rp {customer.current_debt.toLocaleString("id-ID")}
          </div>
        </div>
        <div>
          <span class="text-slate-500">Plafon Kredit Toko:</span>
          <div class="text-base font-bold font-mono text-slate-800 mt-0.5">
            Rp {customer.credit_limit.toLocaleString("id-ID")}
          </div>
        </div>
      </div>

      <!-- Tab Switcher -->
      <div class="flex border-b border-slate-200 bg-slate-100 text-xs">
        <button
          onclick={() => (activeSubTab = "pay")}
          class="flex-1 py-2 font-medium border-b-2 {activeSubTab === 'pay'
            ? 'border-emerald-600 text-emerald-800 bg-white font-semibold'
            : 'border-transparent text-slate-600 hover:text-slate-900'}"
        >
          Catat Pembayaran
        </button>
        <button
          onclick={() => {
            activeSubTab = "history";
            loadHistory(customer.id);
          }}
          class="flex-1 py-2 font-medium border-b-2 {activeSubTab === 'history'
            ? 'border-emerald-600 text-emerald-800 bg-white font-semibold'
            : 'border-transparent text-slate-600 hover:text-slate-900'}"
        >
          Histori Pembayaran ({historyList.length})
        </button>
      </div>

      <!-- Isi Konten Tab -->
      <div class="p-5 flex-1 overflow-y-auto">
        {#if activeSubTab === "pay"}
          <form onsubmit={(e) => { e.preventDefault(); handleSubmitPayment(); }} class="space-y-3.5 text-xs text-slate-700">
            <div>
              <div class="flex justify-between items-center mb-1">
                <label for="pay-amount-input" class="font-semibold text-slate-800">Nominal Pembayaran (Rp) *</label>
                <button
                  type="button"
                  onclick={() => (payAmount = customer.current_debt)}
                  class="text-[11px] text-emerald-700 font-bold hover:underline"
                >
                  Lunasi Penuh (Rp {customer.current_debt.toLocaleString("id-ID")})
                </button>
              </div>
              <input
                id="pay-amount-input"
                type="number"
                min="1"
                max={customer.current_debt}
                bind:value={payAmount}
                required
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-mono font-bold text-base text-slate-900 focus:border-emerald-600 focus:outline-none"
              />
            </div>

            <div>
              <label for="pay-method-select" class="block font-semibold mb-1 text-slate-800">Metode Penyetoran</label>
              <select
                id="pay-method-select"
                bind:value={selectedMethodId}
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-semibold focus:border-emerald-600 focus:outline-none"
              >
                {#each paymentMethods as pm}
                  <option value={pm.id}>{pm.name} ({pm.category})</option>
                {/each}
              </select>
            </div>

            <div>
              <label for="pay-ref-input" class="block font-semibold mb-1 text-slate-800">No. Referensi / Bukti Transfer (Opsional)</label>
              <input
                id="pay-ref-input"
                type="text"
                placeholder="misal: Ref BCA 881923, No. Cek/Bilyet Giro"
                bind:value={referenceNumber}
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg focus:border-emerald-600 focus:outline-none"
              />
            </div>

            <div>
              <label for="pay-notes-input" class="block font-semibold mb-1 text-slate-800">Catatan Pembayaran (Opsional)</label>
              <input
                id="pay-notes-input"
                type="text"
                placeholder="misal: Pembayaran faktur minggu pertama"
                bind:value={notes}
                class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg focus:border-emerald-600 focus:outline-none"
              />
            </div>

            <div class="pt-2 flex justify-between items-center text-xs border-t border-slate-200">
              <span class="text-slate-500">Sisa Piutang Setelah Bayar:</span>
              <span class="font-bold font-mono text-emerald-800 text-sm">
                Rp {Math.max(0, customer.current_debt - payAmount).toLocaleString("id-ID")}
              </span>
            </div>

            <div class="flex justify-end gap-2 pt-3 border-t border-slate-200">
              <button
                type="button"
                onclick={onClose}
                class="px-4 py-2 bg-white hover:bg-slate-100 border border-slate-300 rounded-lg font-medium text-slate-700"
              >
                Batal (ESC)
              </button>
              <button
                type="submit"
                class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg font-bold flex items-center gap-1.5 shadow-xs"
              >
                <Check class="w-4 h-4" />
                <span>Simpan Pembayaran</span>
              </button>
            </div>
          </form>
        {:else if activeSubTab === "history"}
          <div class="space-y-2 text-xs">
            {#if isLoadingHistory}
              <div class="py-8 text-center text-slate-400">Memuat riwayat pembayaran...</div>
            {:else if historyList.length === 0}
              <div class="py-8 text-center text-slate-400">Belum ada riwayat pembayaran piutang untuk mitra ini.</div>
            {:else}
              {#each historyList as h}
                <div class="p-3 bg-slate-50 border border-slate-200 rounded-lg flex justify-between items-start">
                  <div>
                    <div class="flex items-center gap-2">
                      <span class="font-bold font-mono text-slate-900">Rp {h.amount.toLocaleString("id-ID")}</span>
                      <span class="text-[10px] bg-emerald-50 text-emerald-700 border border-emerald-200 px-1 rounded font-semibold">
                        {h.payment_method_name}
                      </span>
                    </div>
                    {#if h.reference_number}
                      <div class="text-[11px] text-slate-500 mt-0.5">Ref: {h.reference_number}</div>
                    {/if}
                    {#if h.notes}
                      <div class="text-[11px] text-slate-600 italic mt-0.5">{h.notes}</div>
                    {/if}
                  </div>
                  <span class="font-mono text-[10px] text-slate-400">
                    {new Date(h.created_at).toLocaleDateString("id-ID")}
                  </span>
                </div>
              {/each}
            {/if}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}
