<script lang="ts">
  import { X, Printer, Coins, Download, CheckCircle, RefreshCw } from "@lucide/svelte";
  import type { TransactionDetail } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    transaction: TransactionDetail | null;
    onClose: () => void;
  }

  let { isOpen = false, transaction = null, onClose }: Props = $props();

  let paperWidth = $state<"58" | "80">("58");
  let isPrinting = $state(false);
  let statusMessage = $state<string | null>(null);

  function handlePrint() {
    isPrinting = true;
    setTimeout(() => {
      window.print();
      isPrinting = false;
    }, 100);
  }

  async function handleKickDrawer() {
    try {
      const msg = await api.kickCashDrawer();
      showStatus(msg || "Laci kas berhasil dibuka!");
    } catch (err) {
      showStatus(`Gagal membuka laci: ${err}`);
    }
  }

  function showStatus(msg: string) {
    statusMessage = msg;
    setTimeout(() => {
      statusMessage = null;
    }, 2500);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "p" && (e.ctrlKey || e.metaKey)) {
      e.preventDefault();
      handlePrint();
    }
  }
</script>

{#if isOpen && transaction}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={handleKeydown}
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4 print:p-0 print:bg-white print:fixed print:inset-0"
  >
    <div
      class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col max-h-[92vh] print:border-none print:shadow-none print:max-w-none print:w-auto print:max-h-none"
    >
      <!-- Header Modal (Hidden Saat Cetak) -->
      <div
        class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center print:hidden"
      >
        <div class="flex items-center gap-2">
          <Printer class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Struk Transaksi Penjualan</h3>
            <p class="text-[11px] text-slate-500 font-mono">{transaction.invoice_number}</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <!-- Toggle Lebar Kertas -->
          <div class="flex bg-slate-200 p-0.5 rounded text-xs font-semibold">
            <button
              type="button"
              onclick={() => (paperWidth = "58")}
              class="px-2 py-0.5 rounded transition-colors {paperWidth === '58'
                ? 'bg-white text-emerald-700 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'}"
            >
              58mm
            </button>
            <button
              type="button"
              onclick={() => (paperWidth = "80")}
              class="px-2 py-0.5 rounded transition-colors {paperWidth === '80'
                ? 'bg-white text-emerald-700 shadow-xs'
                : 'text-slate-600 hover:text-slate-900'}"
            >
              80mm
            </button>
          </div>

          <button
            onclick={onClose}
            class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Pesan Feedback Flash -->
      {#if statusMessage}
        <div
          class="px-4 py-2 bg-emerald-50 border-b border-emerald-200 text-emerald-800 text-xs font-medium flex items-center gap-2 print:hidden"
        >
          <CheckCircle class="w-4 h-4 text-emerald-600" />
          <span>{statusMessage}</span>
        </div>
      {/if}

      <!-- Visual Lembar Kertas Struk Thermal -->
      <div class="flex-1 p-4 overflow-y-auto bg-slate-100 flex justify-center print:p-0 print:bg-white">
        <div
          id="printable-receipt"
          class="bg-white border border-slate-300 shadow-sm p-4 font-mono text-[11px] leading-tight text-slate-900 transition-all {paperWidth === '58'
            ? 'w-[300px]'
            : 'w-[400px]'} print:border-none print:shadow-none print:p-0 print:w-[58mm]"
        >
          <!-- Bagian Atas: Profil Toko -->
          <div class="text-center space-y-0.5 pb-2">
            <h4 class="font-bold text-sm tracking-wide text-slate-900">{transaction.store_info.name}</h4>
            {#if transaction.store_info.branch}
              <div class="text-[11px] text-slate-700">{transaction.store_info.branch}</div>
            {/if}
            {#if transaction.store_info.address}
              <div class="text-[10px] text-slate-600 leading-tight">{transaction.store_info.address}</div>
            {/if}
            {#if transaction.store_info.phone}
              <div class="text-[10px] text-slate-600">Telp: {transaction.store_info.phone}</div>
            {/if}
            {#if transaction.store_info.npwp}
              <div class="text-[10px] text-slate-600">NPWP: {transaction.store_info.npwp}</div>
            {/if}
          </div>

          <!-- Divider Ganda -->
          <div class="border-b-2 border-dashed border-slate-400 my-2"></div>

          <!-- Metadata Transaksi -->
          <div class="space-y-1 text-[10px] text-slate-700">
            <div class="flex justify-between">
              <span>No Nota:</span>
              <span class="font-bold text-slate-900">{transaction.invoice_number}</span>
            </div>
            <div class="flex justify-between">
              <span>Waktu:</span>
              <span>{new Date(transaction.created_at).toLocaleString("id-ID")}</span>
            </div>
            <div class="flex justify-between">
              <span>Kasir:</span>
              <span>{transaction.cashier_name}</span>
            </div>
            <div class="flex justify-between">
              <span>Tipe:</span>
              <span class="uppercase font-semibold">{transaction.order_type}</span>
            </div>
            {#if transaction.customer_name}
              <div class="flex justify-between">
                <span>Pelanggan:</span>
                <span class="font-semibold text-slate-900">{transaction.customer_name}</span>
              </div>
            {/if}
          </div>

          <!-- Divider Item -->
          <div class="border-b border-dashed border-slate-300 my-2"></div>

          <!-- Daftar Baris Barang -->
          <div class="space-y-2 text-[11px]">
            {#each transaction.items as item}
              <div>
                <div class="font-bold text-slate-900 leading-snug">{item.product_name}</div>
                <div class="flex justify-between text-slate-700 text-[10px] mt-0.5">
                  <span>
                    {item.quantity} {item.unit_name} x Rp {item.unit_price.toLocaleString("id-ID")}
                  </span>
                  <span class="font-bold font-mono text-slate-900">
                    Rp {item.subtotal.toLocaleString("id-ID")}
                  </span>
                </div>
              </div>
            {/each}
          </div>

          <!-- Divider Total -->
          <div class="border-b-2 border-dashed border-slate-400 my-2"></div>

          <!-- Rincian Nilai & Pajak -->
          <div class="space-y-1 text-[11px]">
            <div class="flex justify-between">
              <span class="text-slate-600">Subtotal:</span>
              <span class="font-mono">Rp {transaction.subtotal.toLocaleString("id-ID")}</span>
            </div>
            {#if transaction.discount_total > 0}
              <div class="flex justify-between text-slate-600">
                <span>Diskon:</span>
                <span class="font-mono">-Rp {transaction.discount_total.toLocaleString("id-ID")}</span>
              </div>
            {/if}
            {#if transaction.tax_mode === "inclusive"}
              <div class="flex justify-between text-[10px] text-slate-500">
                <span>Termasuk PPN 11%:</span>
                <span class="font-mono">Rp {transaction.tax_total.toLocaleString("id-ID")}</span>
              </div>
            {:else if transaction.tax_mode === "exclusive"}
              <div class="flex justify-between text-[10px] text-slate-500">
                <span>PPN 11% (Tambahan):</span>
                <span class="font-mono">Rp {transaction.tax_total.toLocaleString("id-ID")}</span>
              </div>
            {/if}

            <div class="flex justify-between font-bold text-xs pt-1 border-t border-slate-300 text-slate-900">
              <span>TOTAL AKHIR:</span>
              <span class="font-mono">Rp {transaction.grand_total.toLocaleString("id-ID")}</span>
            </div>

            <!-- Metode Pembayaran & Kembalian -->
            {#each transaction.payments as pay}
              <div class="flex justify-between pt-1 text-[11px] text-slate-700">
                <span>Bayar ({pay.payment_method_name}):</span>
                <span class="font-mono">Rp {pay.amount_paid.toLocaleString("id-ID")}</span>
              </div>
              {#if pay.change_due > 0}
                <div class="flex justify-between text-[11px] text-slate-700">
                  <span>Kembali:</span>
                  <span class="font-mono font-bold text-slate-900">Rp {pay.change_due.toLocaleString("id-ID")}</span>
                </div>
              {/if}
            {/each}
          </div>

          <!-- Divider Footer -->
          <div class="border-b border-dashed border-slate-300 my-2.5"></div>

          <!-- Catatan Kaki / Footer -->
          <div class="text-center text-[10px] text-slate-600 whitespace-pre-line leading-relaxed">
            {transaction.store_info.footer}
          </div>

          <div class="text-center text-[9px] text-slate-400 mt-2">
            *** POS Kasir Desktop System ***
          </div>
        </div>
      </div>

      <!-- Action Footer (Hidden Saat Cetak) -->
      <div class="px-5 py-3.5 bg-slate-50 border-t border-slate-200 flex justify-between items-center print:hidden">
        <button
          type="button"
          onclick={handleKickDrawer}
          class="px-3 py-2 bg-slate-200 hover:bg-slate-300 text-slate-800 font-semibold text-xs rounded-lg transition-colors flex items-center gap-1.5 border border-slate-300"
          title="Kirim sinyal pulse untuk membuka laci kasir RJ11"
        >
          <Coins class="w-4 h-4 text-amber-600" />
          <span>Buka Laci Kas</span>
        </button>

        <div class="flex gap-2">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 bg-white hover:bg-slate-100 text-slate-700 font-medium text-xs rounded-lg transition-colors border border-slate-300"
          >
            Tutup (ESC)
          </button>
          <button
            type="button"
            onclick={handlePrint}
            class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-lg transition-colors flex items-center gap-1.5 shadow-xs"
          >
            <Printer class="w-4 h-4" />
            <span>Cetak Struk Sekarang</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  @media print {
    :global(body) {
      margin: 0 !important;
      padding: 0 !important;
      background: white !important;
    }
    @page {
      size: 58mm auto;
      margin: 0;
    }
  }
</style>
