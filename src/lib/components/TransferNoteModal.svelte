<script lang="ts">
  import { X, Printer, ArrowRight, Building2, CheckCircle2, Truck } from "@lucide/svelte";
  import type { InterBranchTransferDetail } from "$lib/types";

  interface Props {
    isOpen: boolean;
    transferDetail: InterBranchTransferDetail | null;
    onClose: () => void;
  }

  let { isOpen = false, transferDetail = null, onClose }: Props = $props();

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

{#if isOpen && transferDetail}
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
      <!-- Header Dialog (Hidden Saat Print) -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center print:hidden">
        <div class="flex items-center gap-2">
          <Truck class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Surat Jalan Mutasi Antar Cabang (Inter-Branch Transfer Note)</h3>
            <p class="text-[11px] text-slate-500 font-mono">{transferDetail.transfer.transfer_number}</p>
          </div>
        </div>
        <div class="flex items-center gap-2">
          <button
            type="button"
            onclick={handlePrint}
            class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold flex items-center gap-1.5 shadow-xs transition-colors"
          >
            <Printer class="w-4 h-4" />
            <span>Cetak Dokumen (Ctrl+P)</span>
          </button>
          <button
            type="button"
            onclick={onClose}
            class="text-slate-400 hover:text-slate-600 p-1.5 rounded-lg hover:bg-slate-100 transition-colors"
          >
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- KERTAS A4 DOKUMEN MUTASI -->
      <div class="p-8 overflow-y-auto flex-1 font-sans text-slate-800 bg-white print:p-0 print:overflow-visible">
        <!-- KOP DOKUMEN MUTASI -->
        <div class="border-b-2 border-slate-900 pb-4 mb-4 flex justify-between items-start">
          <div>
            <span class="text-[11px] uppercase tracking-widest font-bold text-emerald-700">DOKUMEN RESMI LOGISTIK INTERNAL</span>
            <h1 class="text-xl font-black text-slate-900 tracking-tight uppercase">SURAT JALAN MUTASI STOK ANTAR CABANG</h1>
            <p class="text-xs text-slate-600 font-medium">Sistem Distribusi Logistik Pergudangan Retail & Grosir Terpadu</p>
          </div>
          <div class="text-right">
            <div class="text-xs font-semibold text-slate-500">Nomor Bukti Transfer:</div>
            <div class="text-sm font-mono font-black text-slate-900 bg-slate-100 px-2 py-0.5 rounded border border-slate-300 inline-block mt-0.5">
              {transferDetail.transfer.transfer_number}
            </div>
            <div class="text-xs text-slate-500 mt-1">
              Tanggal: <span class="font-mono text-slate-800">{new Date(transferDetail.transfer.created_at).toLocaleDateString("id-ID", { day: "2-digit", month: "long", year: "numeric" })}</span>
            </div>
          </div>
        </div>

        <!-- BLOK CABANG PENGIRIM & CABANG PENERIMA -->
        <div class="grid grid-cols-2 gap-4 p-3.5 bg-slate-50 border border-slate-300 rounded-lg text-xs mb-4">
          <!-- Sisi Kiri: Cabang Pengirim -->
          <div class="space-y-1 border-r border-slate-200 pr-3">
            <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider flex items-center gap-1">
              <Building2 class="w-3.5 h-3.5 text-emerald-600" />
              <span>CABANG ASAL (PENGIRIM)</span>
            </div>
            <div class="font-bold text-slate-900 text-sm">{transferDetail.source_branch.name}</div>
            <div class="text-slate-600 font-mono text-[11px]">Kode Cabang: {transferDetail.source_branch.code}</div>
            <div class="text-slate-600">{transferDetail.source_branch.address || "-"}</div>
            <div class="text-slate-600">Kontak: {transferDetail.source_branch.phone || "-"}</div>
          </div>

          <!-- Sisi Kanan: Cabang Penerima -->
          <div class="space-y-1 pl-1">
            <div class="text-[10px] font-bold text-slate-500 uppercase tracking-wider flex items-center gap-1">
              <Building2 class="w-3.5 h-3.5 text-blue-600" />
              <span>CABANG TUJUAN (PENERIMA)</span>
            </div>
            <div class="font-bold text-slate-900 text-sm">{transferDetail.destination_branch.name}</div>
            <div class="text-slate-600 font-mono text-[11px]">Kode Cabang: {transferDetail.destination_branch.code}</div>
            <div class="text-slate-600">{transferDetail.destination_branch.address || "-"}</div>
            <div class="text-slate-600">Kontak: {transferDetail.destination_branch.phone || "-"}</div>
          </div>
        </div>

        <!-- STATUS TRANSFER & CATATAN -->
        <div class="flex justify-between items-center text-xs px-1 mb-3">
          <div class="flex items-center gap-2">
            <span class="text-slate-500 font-semibold">Status Dokumen:</span>
            <span class="px-2 py-0.5 rounded text-[11px] font-bold uppercase {transferDetail.transfer.status === 'received' ? 'bg-emerald-100 text-emerald-800 border border-emerald-300' : 'bg-amber-100 text-amber-800 border border-amber-300'}">
              {transferDetail.transfer.status === 'received' ? 'Diterima & Stok Masuk' : 'Dalam Perjalanan (In Transit)'}
            </span>
          </div>
          <div class="text-slate-500 italic text-[11px]">
            {transferDetail.transfer.notes ? `Catatan: ${transferDetail.transfer.notes}` : "Kondisi muatan: Tersegel utuh"}
          </div>
        </div>

        <!-- TABEL RINCIAN FISIK BARANG TRANSFER -->
        <table class="w-full text-xs border-collapse border border-slate-300 mb-6">
          <thead class="bg-slate-100 text-slate-700 font-bold uppercase text-[11px]">
            <tr>
              <th class="border border-slate-300 py-2 px-2.5 text-center w-10">No</th>
              <th class="border border-slate-300 py-2 px-3 text-left w-28">SKU / Kode</th>
              <th class="border border-slate-300 py-2 px-3 text-left">Deskripsi Produk</th>
              <th class="border border-slate-300 py-2 px-3 text-center w-28">Kemasan / Satuan</th>
              <th class="border border-slate-300 py-2 px-3 text-center w-24">Jumlah Kirim</th>
              <th class="border border-slate-300 py-2 px-3 text-center w-28">Konversi Fisik</th>
              <th class="border border-slate-300 py-2 px-2 text-center w-16">Cek Fisik</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-200">
            {#each transferDetail.items as item, idx}
              <tr class="hover:bg-slate-50">
                <td class="border border-slate-300 py-2 px-2.5 text-center font-mono text-slate-500">{idx + 1}</td>
                <td class="border border-slate-300 py-2 px-3 font-mono text-slate-600">{item.sku}</td>
                <td class="border border-slate-300 py-2 px-3 font-semibold text-slate-900">{item.product_name}</td>
                <td class="border border-slate-300 py-2 px-3 text-center text-slate-700 font-medium">{item.unit_name}</td>
                <td class="border border-slate-300 py-2 px-3 text-center font-mono font-bold text-slate-900 text-sm">
                  {item.quantity}
                </td>
                <td class="border border-slate-300 py-2 px-3 text-center font-mono text-slate-700">
                  {item.base_qty} Pcs/Dasar
                </td>
                <td class="border border-slate-300 py-2 px-2 text-center">
                  <div class="w-4 h-4 border border-slate-400 rounded-xs mx-auto"></div>
                </td>
              </tr>
            {/each}
          </tbody>
          <tfoot class="bg-slate-50 font-semibold text-slate-700">
            <tr>
              <td colspan="4" class="border border-slate-300 py-2 px-3 text-right">TOTAL BARANG MUTASI:</td>
              <td class="border border-slate-300 py-2 px-3 text-center font-bold font-mono text-slate-900">
                {transferDetail.items.reduce((sum, i) => sum + i.quantity, 0)} Kemasan
              </td>
              <td class="border border-slate-300 py-2 px-3 text-center font-bold font-mono text-slate-900">
                {transferDetail.items.reduce((sum, i) => sum + i.base_qty, 0)} Total Fisik
              </td>
              <td class="border border-slate-300"></td>
            </tr>
          </tfoot>
        </table>

        <!-- BLOK 3 TANDA TANGAN SERAH TERIMA -->
        <div class="grid grid-cols-3 gap-6 pt-4 text-xs text-center border-t border-slate-200">
          <div>
            <div class="text-[11px] font-semibold text-slate-600 mb-1">Pengirim (Gudang Asal)</div>
            <div class="h-20 border-b border-dashed border-slate-400 flex items-end justify-center pb-1 text-slate-400 italic">
              {transferDetail.transfer.sent_by_name || "Petugas Cabang Pengirim"}
            </div>
            <div class="font-bold text-slate-800 mt-1">({transferDetail.source_branch.name})</div>
          </div>

          <div>
            <div class="text-[11px] font-semibold text-slate-600 mb-1">Kurir / Ekspedisi Pengantar</div>
            <div class="h-20 border-b border-dashed border-slate-400 flex items-end justify-center pb-1 text-slate-400 italic">
              Nama Terang & No. Armada
            </div>
            <div class="font-bold text-slate-800 mt-1">(Sopir / Jasa Angkut)</div>
          </div>

          <div>
            <div class="text-[11px] font-semibold text-slate-600 mb-1">Penerima (Gudang Tujuan)</div>
            <div class="h-20 border-b border-dashed border-slate-400 flex items-end justify-center pb-1 text-slate-400 italic">
              {transferDetail.transfer.received_by_name || "Tanda Tangan & Stempel"}
            </div>
            <div class="font-bold text-slate-800 mt-1">({transferDetail.destination_branch.name})</div>
          </div>
        </div>

        <div class="mt-8 text-center text-[10px] text-slate-400 border-t border-slate-100 pt-2">
          Surat jalan mutasi internal diterbitkan otomatis oleh sistem POS Kasir Terdistribusi (Mode WAL + LAN Sync)
        </div>
      </div>
    </div>
  </div>
{/if}
