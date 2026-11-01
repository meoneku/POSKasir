<script lang="ts">
  import { X, Printer, Truck, Check, MapPin, Building2 } from "@lucide/svelte";
  import type { DeliveryOrderDetail, TransactionDetail } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    transaction: TransactionDetail | null;
    onClose: () => void;
  }

  let { isOpen = false, transaction = null, onClose }: Props = $props();

  let driverName = $state("");
  let vehicleNumber = $state("");
  let notes = $state("");
  let deliveryOrder = $state<DeliveryOrderDetail | null>(null);
  let isLoading = $state(false);

  $effect(() => {
    if (isOpen && transaction) {
      driverName = "Pak Joko (Armada Toko)";
      vehicleNumber = "B 9876 POS";
      notes = "Barang telah dicek dan dimuat dalam kondisi baik dan tersegel.";
      loadDeliveryOrder(transaction.id);
    }
  });

  async function loadDeliveryOrder(txId: string) {
    isLoading = true;
    try {
      deliveryOrder = await api.getDeliveryOrder(
        txId,
        driverName || undefined,
        vehicleNumber || undefined,
        notes || undefined
      );
    } catch (err) {
      console.error("Gagal memuat surat jalan:", err);
    } finally {
      isLoading = false;
    }
  }

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
      <!-- Header Dialog (Hidden Saat Cetak) -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center print:hidden">
        <div class="flex items-center gap-2">
          <Truck class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Surat Jalan Pengiriman (Delivery Order / DO)</h3>
            <p class="text-[11px] text-slate-500 font-mono">{transaction.invoice_number.replace('INV/', 'DO/')}</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={handlePrint}
            class="px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-lg transition-colors flex items-center gap-1.5 shadow-xs"
          >
            <Printer class="w-4 h-4" />
            <span>Cetak Surat Jalan (Ctrl+P)</span>
          </button>
          <button
            onclick={onClose}
            class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Pengaturan Supir & Kendaraan (Hidden Saat Cetak) -->
      <div class="px-6 py-3 bg-slate-100 border-b border-slate-200 grid grid-cols-3 gap-3 text-xs print:hidden">
        <div>
          <label for="do-driver-input" class="block font-semibold text-slate-700 mb-0.5">Nama Supir / Kurir</label>
          <input
            id="do-driver-input"
            type="text"
            bind:value={driverName}
            class="w-full px-2.5 py-1.5 bg-white border border-slate-300 rounded text-slate-900"
          />
        </div>
        <div>
          <label for="do-nopol-input" class="block font-semibold text-slate-700 mb-0.5">No. Polisi Kendaraan</label>
          <input
            id="do-nopol-input"
            type="text"
            bind:value={vehicleNumber}
            class="w-full px-2.5 py-1.5 bg-white border border-slate-300 rounded text-slate-900 font-mono"
          />
        </div>
        <div>
          <label for="do-notes-input" class="block font-semibold text-slate-700 mb-0.5">Catatan Pengiriman</label>
          <input
            id="do-notes-input"
            type="text"
            bind:value={notes}
            class="w-full px-2.5 py-1.5 bg-white border border-slate-300 rounded text-slate-900"
          />
        </div>
      </div>

      <!-- Lembar Dokumen Surat Jalan Format A4 -->
      <div class="flex-1 p-6 overflow-y-auto bg-slate-100 flex justify-center print:p-0 print:bg-white">
        <div
          id="printable-do"
          class="bg-white border border-slate-300 shadow-sm p-8 text-slate-900 w-[780px] min-h-[1050px] flex flex-col justify-between print:border-none print:shadow-none print:p-0 print:w-full print:min-h-0"
        >
          <!-- Bagian Atas: Kop Toko & Info Pengiriman -->
          <div class="space-y-4">
            <div class="flex justify-between items-start border-b-2 border-slate-900 pb-4">
              <div>
                <h1 class="text-xl font-bold tracking-tight text-slate-900 uppercase">
                  {transaction.store_info.name}
                </h1>
                <p class="text-xs text-slate-600 font-medium">{transaction.store_info.branch}</p>
                <p class="text-[11px] text-slate-600 max-w-sm mt-1 leading-snug">{transaction.store_info.address}</p>
                <p class="text-[11px] text-slate-600 mt-1">Telp: {transaction.store_info.phone}</p>
              </div>

              <div class="text-right">
                <div class="inline-block bg-slate-900 text-white px-3 py-1 rounded font-bold text-sm tracking-wide uppercase">
                  SURAT JALAN (DO)
                </div>
                <div class="text-xs font-mono font-bold text-slate-900 mt-2">
                  No. DO: {transaction.invoice_number.replace('INV/', 'DO/')}
                </div>
                <div class="text-[11px] text-slate-600 mt-0.5">
                  Ref Faktur: <span class="font-mono font-semibold">{transaction.invoice_number}</span>
                </div>
                <div class="text-[11px] text-slate-600 mt-0.5">
                  Tanggal Kirim: {new Date(transaction.created_at).toLocaleDateString("id-ID", { day: "numeric", month: "long", year: "numeric" })}
                </div>
              </div>
            </div>

            <!-- Identitas Penerima Barang & Ekspedisi -->
            <div class="bg-slate-50 border border-slate-200 rounded p-3 text-xs grid grid-cols-2 gap-4">
              <div>
                <span class="text-[10px] uppercase font-bold text-slate-500 tracking-wider">Tujuan Pengiriman:</span>
                <h4 class="font-bold text-slate-900 text-sm mt-0.5">{transaction.customer_name || "Pelanggan Umum"}</h4>
                {#if transaction.customer_address}
                  <p class="text-[11px] text-slate-600 mt-0.5 leading-snug">{transaction.customer_address}</p>
                {/if}
                {#if transaction.customer_phone}
                  <p class="text-[11px] text-slate-600 mt-0.5">Kontak: {transaction.customer_phone}</p>
                {/if}
              </div>
              <div class="text-right space-y-1">
                <div>
                  <span class="text-slate-500">Nama Pengemudi / Kurir:</span>
                  <span class="font-bold text-slate-900 ml-1">{driverName}</span>
                </div>
                <div>
                  <span class="text-slate-500">No. Polisi Armada:</span>
                  <span class="font-mono font-bold text-slate-900 ml-1">{vehicleNumber}</span>
                </div>
                {#if notes}
                  <div class="text-slate-600 italic mt-1 text-[11px]">
                    "{notes}"
                  </div>
                {/if}
              </div>
            </div>

            <!-- Catatan Tanpa Rupiah -->
            <div class="p-2 bg-amber-50 border border-amber-200 rounded text-[11px] text-amber-900 font-medium">
              Dokumen resmi serah terima fisik barang. Pastikan seluruh jumlah dan kemasan barang telah diperiksa dengan seksama sebelum menandatangani bukti terima.
            </div>

            <!-- Tabel Daftar Fisik Barang (Tanpa Nilai Rupiah) -->
            <table class="w-full text-left text-xs border border-slate-300 border-collapse mt-2">
              <thead class="bg-slate-100 text-slate-800 border-b border-slate-300 font-bold uppercase text-[11px]">
                <tr>
                  <th class="py-2.5 px-2.5 border-r border-slate-300 text-center w-10">No</th>
                  <th class="py-2.5 px-3 border-r border-slate-300">Deskripsi Barang</th>
                  <th class="py-2.5 px-3 border-r border-slate-300 text-center w-24">Satuan</th>
                  <th class="py-2.5 px-3 border-r border-slate-300 text-center w-28">Kuantitas Kirim</th>
                  <th class="py-2.5 px-3 text-center w-36">Cek Fisik Diterima</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-200 text-slate-900 text-xs">
                {#each transaction.items as item, index}
                  <tr class="hover:bg-slate-50">
                    <td class="py-2.5 px-2.5 border-r border-slate-200 text-center font-mono">{index + 1}</td>
                    <td class="py-2.5 px-3 border-r border-slate-200 font-semibold text-slate-900">
                      {item.product_name}
                    </td>
                    <td class="py-2.5 px-3 border-r border-slate-200 text-center font-medium">{item.unit_name}</td>
                    <td class="py-2.5 px-3 border-r border-slate-200 text-center font-mono font-bold text-sm">
                      {item.quantity}
                    </td>
                    <td class="py-2.5 px-3 text-center">
                      <div class="w-6 h-6 border-2 border-slate-400 rounded mx-auto flex items-center justify-center">
                        <span class="text-slate-300 text-xs">&#10003;</span>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>

          <!-- Bagian Bawah: Kotak Tanda Tangan 3 Pihak -->
          <div class="pt-12 border-t border-slate-200 grid grid-cols-3 gap-6 text-center text-xs text-slate-700">
            <div>
              <p class="font-semibold text-slate-800">Penerima Barang,</p>
              <div class="h-24 flex items-center justify-center text-[10px] text-slate-400">
                (Tanda Tangan & Stempel Toko)
              </div>
              <p class="font-mono text-slate-800 border-t border-slate-400 pt-1 font-bold">
                ( {transaction.customer_name || "Pelanggan"} )
              </p>
            </div>

            <div>
              <p class="font-semibold text-slate-800">Sopir / Kurir Pengantar,</p>
              <div class="h-24"></div>
              <p class="font-mono text-slate-800 border-t border-slate-400 pt-1 font-bold">
                ( {driverName} )
              </p>
            </div>

            <div>
              <p class="font-semibold text-slate-800">Petugas Gudang Toko,</p>
              <div class="h-24"></div>
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
