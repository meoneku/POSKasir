<script lang="ts">
  import { X, Printer, FileText, CheckCircle, Building2 } from "@lucide/svelte";
  import type { TransactionDetail } from "$lib/types";

  interface Props {
    isOpen: boolean;
    transaction: TransactionDetail | null;
    onClose: () => void;
  }

  let { isOpen = false, transaction = null, onClose }: Props = $props();

  function handlePrint() {
    window.print();
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
      class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-4xl overflow-hidden flex flex-col max-h-[95vh] print:border-none print:shadow-none print:max-w-none print:w-full print:max-h-none"
    >
      <!-- Header Modal (Hidden Saat Cetak) -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center print:hidden">
        <div class="flex items-center gap-2">
          <FileText class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Faktur Penjualan & Pajak (Invoice A4 / NCR)</h3>
            <p class="text-[11px] text-slate-500 font-mono">{transaction.invoice_number}</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={handlePrint}
            class="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-lg transition-colors flex items-center gap-1.5 shadow-xs"
          >
            <Printer class="w-4 h-4" />
            <span>Cetak Faktur A4 (Ctrl+P)</span>
          </button>
          <button
            onclick={onClose}
            class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Lembar Dokumen Faktur Format A4 -->
      <div class="flex-1 p-6 overflow-y-auto bg-slate-100 flex justify-center print:p-0 print:bg-white">
        <div
          id="printable-invoice"
          class="bg-white border border-slate-300 shadow-sm p-8 text-slate-900 w-[780px] min-h-[1050px] flex flex-col justify-between print:border-none print:shadow-none print:p-0 print:w-full print:min-h-0"
        >
          <!-- Bagian Atas: Kop Toko & Info Faktur -->
          <div class="space-y-4">
            <div class="flex justify-between items-start border-b-2 border-slate-900 pb-4">
              <div>
                <h1 class="text-xl font-bold tracking-tight text-slate-900 uppercase">
                  {transaction.store_info.name}
                </h1>
                <p class="text-xs text-slate-600 font-medium">{transaction.store_info.branch}</p>
                <p class="text-[11px] text-slate-600 max-w-sm mt-1 leading-snug">{transaction.store_info.address}</p>
                <div class="text-[11px] text-slate-600 mt-1 flex gap-4">
                  <span>Telp: {transaction.store_info.phone}</span>
                  {#if transaction.store_info.npwp}
                    <span>NPWP: {transaction.store_info.npwp}</span>
                  {/if}
                </div>
              </div>

              <div class="text-right">
                <div class="inline-block bg-slate-100 px-3 py-1 border border-slate-300 rounded font-bold text-sm tracking-wide uppercase text-slate-900">
                  FAKTUR PENJUALAN
                </div>
                <div class="text-xs font-mono font-bold text-slate-900 mt-2">
                  No: {transaction.invoice_number}
                </div>
                <div class="text-[11px] text-slate-600 mt-0.5">
                  Tanggal: {new Date(transaction.created_at).toLocaleDateString("id-ID", { day: "numeric", month: "long", year: "numeric" })}
                </div>
                {#if transaction.due_date}
                  <div class="text-[11px] text-rose-700 font-bold mt-0.5">
                    Jatuh Tempo: {new Date(transaction.due_date).toLocaleDateString("id-ID", { day: "numeric", month: "long", year: "numeric" })}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Identitas Pembeli (Customer B2B) -->
            <div class="bg-slate-50 border border-slate-200 rounded p-3 text-xs grid grid-cols-2 gap-4">
              <div>
                <span class="text-[10px] uppercase font-bold text-slate-500 tracking-wider">Kepada Yth. (Pembeli / Mitra):</span>
                <h4 class="font-bold text-slate-900 text-sm mt-0.5">{transaction.customer_name || "Pelanggan Umum"}</h4>
                {#if transaction.customer_address}
                  <p class="text-[11px] text-slate-600 mt-0.5 leading-snug">{transaction.customer_address}</p>
                {/if}
                {#if transaction.customer_phone}
                  <p class="text-[11px] text-slate-600 mt-0.5">Telp: {transaction.customer_phone}</p>
                {/if}
              </div>
              <div class="text-right space-y-1">
                {#if transaction.customer_npwp_nik}
                  <div>
                    <span class="text-slate-500">NPWP / NIK Pembeli:</span>
                    <span class="font-mono font-semibold text-slate-800 ml-1">{transaction.customer_npwp_nik}</span>
                  </div>
                {/if}
                <div>
                  <span class="text-slate-500">Syarat Pembayaran:</span>
                  <span class="font-bold text-slate-900 ml-1 uppercase">
                    {transaction.payment_status === "paid" ? "Lunas" : "Tempo / Piutang Usaha"}
                  </span>
                </div>
                <div>
                  <span class="text-slate-500">Kasir / Petugas:</span>
                  <span class="font-semibold text-slate-800 ml-1">{transaction.cashier_name}</span>
                </div>
              </div>
            </div>

            <!-- Tabel Rincian Barang B2B -->
            <table class="w-full text-left text-xs border border-slate-300 border-collapse mt-3">
              <thead class="bg-slate-100 text-slate-800 border-b border-slate-300 font-bold uppercase text-[11px]">
                <tr>
                  <th class="py-2 px-2.5 border-r border-slate-300 text-center w-10">No</th>
                  <th class="py-2 px-3 border-r border-slate-300">Nama Barang / Deskripsi</th>
                  <th class="py-2 px-2.5 border-r border-slate-300 text-center w-20">Satuan</th>
                  <th class="py-2 px-2.5 border-r border-slate-300 text-right w-16">Qty</th>
                  <th class="py-2 px-3 border-r border-slate-300 text-right w-28">Harga Satuan</th>
                  <th class="py-2 px-3 border-r border-slate-300 text-right w-20">Diskon</th>
                  <th class="py-2 px-3 text-right w-32">Subtotal (Rp)</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-200 text-slate-900 text-xs">
                {#each transaction.items as item, index}
                  <tr class="hover:bg-slate-50">
                    <td class="py-2 px-2.5 border-r border-slate-200 text-center font-mono">{index + 1}</td>
                    <td class="py-2 px-3 border-r border-slate-200">
                      <div class="font-semibold text-slate-900">{item.product_name}</div>
                      {#if item.is_taxable}
                        <span class="text-[9px] bg-emerald-50 text-emerald-700 px-1 rounded border border-emerald-200 font-semibold">BKP</span>
                      {:else}
                        <span class="text-[9px] bg-slate-100 text-slate-500 px-1 rounded border border-slate-200">Non-BKP</span>
                      {/if}
                    </td>
                    <td class="py-2 px-2.5 border-r border-slate-200 text-center">{item.unit_name}</td>
                    <td class="py-2 px-2.5 border-r border-slate-200 text-right font-mono font-bold">{item.quantity}</td>
                    <td class="py-2 px-3 border-r border-slate-200 text-right font-mono">
                      Rp {item.unit_price.toLocaleString("id-ID")}
                    </td>
                    <td class="py-2 px-3 border-r border-slate-200 text-right font-mono text-slate-500">
                      {item.discount > 0 ? `Rp ${item.discount.toLocaleString("id-ID")}` : "-"}
                    </td>
                    <td class="py-2 px-3 text-right font-mono font-bold">
                      Rp {item.subtotal.toLocaleString("id-ID")}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>

            <!-- Rekapitulasi Nilai & Terbilang -->
            <div class="grid grid-cols-12 gap-4 pt-2">
              <div class="col-span-7 bg-slate-50 border border-slate-200 rounded p-3 flex flex-col justify-between">
                <div>
                  <span class="text-[10px] uppercase font-bold text-slate-500">Terbilang (Nominal Kata):</span>
                  <p class="font-semibold italic text-slate-900 text-xs mt-1 leading-snug">
                    # {transaction.terbilang} #
                  </p>
                </div>
                {#if transaction.store_info.footer}
                  <div class="text-[10px] text-slate-500 pt-2 border-t border-slate-200 mt-2">
                    {transaction.store_info.footer}
                  </div>
                {/if}
              </div>

              <div class="col-span-5 space-y-1 text-xs">
                <div class="flex justify-between py-0.5">
                  <span class="text-slate-600">Subtotal:</span>
                  <span class="font-mono">Rp {transaction.subtotal.toLocaleString("id-ID")}</span>
                </div>
                {#if transaction.discount_total > 0}
                  <div class="flex justify-between py-0.5 text-slate-600">
                    <span>Diskon Faktur:</span>
                    <span class="font-mono">-Rp {transaction.discount_total.toLocaleString("id-ID")}</span>
                  </div>
                {/if}
                {#if transaction.tax_mode === "inclusive"}
                  <div class="flex justify-between py-0.5 text-slate-500 text-[11px]">
                    <span>Termasuk PPN 11%:</span>
                    <span class="font-mono">Rp {transaction.tax_total.toLocaleString("id-ID")}</span>
                  </div>
                {:else if transaction.tax_mode === "exclusive"}
                  <div class="flex justify-between py-0.5 text-slate-600">
                    <span>PPN 11% (Tambahan):</span>
                    <span class="font-mono">Rp {transaction.tax_total.toLocaleString("id-ID")}</span>
                  </div>
                {/if}
                <div class="flex justify-between items-baseline pt-2 border-t-2 border-slate-900 text-slate-900">
                  <span class="font-bold text-sm">TOTAL TAGIHAN:</span>
                  <span class="font-bold text-base font-mono text-emerald-700">
                    Rp {transaction.grand_total.toLocaleString("id-ID")}
                  </span>
                </div>
              </div>
            </div>
          </div>

          <!-- Bagian Bawah: Kotak Tanda Tangan Formal 3 Kolom -->
          <div class="pt-10 border-t border-slate-200 grid grid-cols-3 gap-6 text-center text-xs text-slate-700">
            <div>
              <p class="font-semibold text-slate-800">Diterima Oleh,</p>
              <div class="h-20 flex items-center justify-center text-[10px] text-slate-400">
                (Stempel & TTD Toko Rekanan)
              </div>
              <p class="font-mono text-slate-800 border-t border-slate-400 pt-1 font-bold">
                ( {transaction.customer_name || "Pelanggan"} )
              </p>
            </div>

            <div>
              <p class="font-semibold text-slate-800">Bagian Gudang / Pengirim,</p>
              <div class="h-20"></div>
              <p class="font-mono text-slate-800 border-t border-slate-400 pt-1 font-bold">
                ( &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; )
              </p>
            </div>

            <div>
              <p class="font-semibold text-slate-800">Kasir / Keuangan,</p>
              <div class="h-20"></div>
              <p class="font-mono text-slate-800 border-t border-slate-400 pt-1 font-bold">
                ( {transaction.cashier_name} )
              </p>
            </div>
          </div>
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
      size: A4 portrait;
      margin: 10mm;
    }
  }
</style>
