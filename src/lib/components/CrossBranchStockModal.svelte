<script lang="ts">
  import { X, Search, Boxes, Building2, Phone, MapPin, RefreshCw, Send } from "@lucide/svelte";
  import type { Product, BranchStockInfo } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    product: Product | null;
    products: Product[];
    onClose: () => void;
    onRequestTransfer: (product: Product, sourceBranchId?: string) => void;
  }

  let {
    isOpen = false,
    product = null,
    products = [],
    onClose,
    onRequestTransfer,
  }: Props = $props();

  let selectedProductId = $state("");
  let branchStocks = $state<BranchStockInfo[]>([]);
  let isLoading = $state(false);

  let activeProduct = $derived(
    products.find((p) => p.id === selectedProductId) || product
  );

  $effect(() => {
    if (isOpen && product) {
      selectedProductId = product.id;
      loadBranchStocks(product.id);
    }
  });

  async function loadBranchStocks(prodId: string) {
    if (!prodId) return;
    isLoading = true;
    try {
      branchStocks = await api.getCrossBranchStock(prodId);
    } catch (err) {
      console.error("Gagal memuat stok cabang lain:", err);
    } finally {
      isLoading = false;
    }
  }

  function handleProductChange(newId: string) {
    selectedProductId = newId;
    loadBranchStocks(newId);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
  }
</script>

{#if isOpen && activeProduct}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={handleKeydown}
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div
      class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-2xl overflow-hidden flex flex-col max-h-[90vh]"
    >
      <!-- Header Modal -->
      <div class="px-5 py-4 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Boxes class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Cek Ketersediaan Stok Lintas Cabang (Cross-Branch)</h3>
            <p class="text-xs text-slate-500">Pantau ketersediaan stok produk di seluruh cabang dan gudang pusat</p>
          </div>
        </div>
        <button
          type="button"
          onclick={onClose}
          class="text-slate-400 hover:text-slate-600 p-1 rounded-lg hover:bg-slate-100 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-4 overflow-y-auto flex-1">
        <!-- Pemilih Produk & Info SKU -->
        <div class="p-3 bg-slate-50 border border-slate-200 rounded-lg flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
          <div class="flex-1">
            <label for="cross-stock-product" class="block text-[11px] font-semibold text-slate-500 uppercase tracking-wide mb-1">
              Pilih Produk untuk Diperiksa:
            </label>
            <select
              id="cross-stock-product"
              value={selectedProductId}
              onchange={(e) => handleProductChange(e.currentTarget.value)}
              class="w-full text-xs bg-white border border-slate-300 rounded-lg px-3 py-1.5 font-bold text-slate-900 focus:outline-none focus:ring-1 focus:ring-emerald-500"
            >
              {#each products as p}
                <option value={p.id}>
                  {p.name} ({p.sku}) - {p.current_stock} {p.base_unit_name}
                </option>
              {/each}
            </select>
          </div>

          <div class="text-right shrink-0">
            <span class="text-[11px] text-slate-500 block">Total Stok Master Tergabung:</span>
            <span class="text-base font-bold font-mono text-emerald-700">
              {branchStocks.reduce((sum, b) => sum + b.stock_qty, 0)} {activeProduct.base_unit_name}
            </span>
          </div>
        </div>

        <!-- Tabel Cabang & Sisa Stok -->
        <div class="space-y-2">
          <div class="flex justify-between items-center">
            <span class="text-xs font-bold text-slate-700 uppercase tracking-wide">
              Lokasi Toko & Gudang Cabang ({branchStocks.length} Cabang Terhubung)
            </span>
            <button
              type="button"
              onclick={() => loadBranchStocks(selectedProductId)}
              class="text-xs text-emerald-700 hover:text-emerald-800 font-semibold flex items-center gap-1"
            >
              <RefreshCw class="w-3.5 h-3.5 {isLoading ? 'animate-spin' : ''}" />
              <span>Muat Ulang</span>
            </button>
          </div>

          <div class="border border-slate-200 rounded-lg overflow-hidden shadow-2xs">
            <table class="w-full text-left text-xs border-collapse">
              <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold">
                <tr>
                  <th class="py-2.5 px-3">Kode</th>
                  <th class="py-2.5 px-3">Nama Toko Cabang</th>
                  <th class="py-2.5 px-3">Kontak / Alamat</th>
                  <th class="py-2.5 px-3 text-right">Sisa Stok</th>
                  <th class="py-2.5 px-3 text-center">Aksi Mutasi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-100 text-slate-800">
                {#each branchStocks as b}
                  <tr class="hover:bg-slate-50 {b.is_current_branch ? 'bg-emerald-50/40' : ''}">
                    <td class="py-2.5 px-3 font-mono font-bold text-slate-900">
                      {b.branch_code}
                    </td>
                    <td class="py-2.5 px-3">
                      <div class="font-bold text-slate-900 flex items-center gap-1.5">
                        <span>{b.branch_name}</span>
                        {#if b.is_current_branch}
                          <span class="text-[10px] bg-emerald-100 text-emerald-800 px-1.5 py-0.2 rounded font-bold border border-emerald-300">
                            Cabang Ini
                          </span>
                        {/if}
                      </div>
                    </td>
                    <td class="py-2.5 px-3 text-slate-600">
                      <div class="truncate max-w-xs">{b.address || "-"}</div>
                      <div class="font-mono text-[11px] text-slate-500">{b.phone || "-"}</div>
                    </td>
                    <td class="py-2.5 px-3 text-right font-mono font-bold text-sm">
                      <span class="{b.stock_qty <= 5 ? 'text-amber-600' : 'text-emerald-700'}">
                        {b.stock_qty} {b.base_unit_name}
                      </span>
                    </td>
                    <td class="py-2.5 px-3 text-center">
                      {#if !b.is_current_branch}
                        <button
                          type="button"
                          onclick={() => {
                            onClose();
                            onRequestTransfer(activeProduct, b.branch_id);
                          }}
                          class="px-2.5 py-1 bg-emerald-50 hover:bg-emerald-100 text-emerald-800 border border-emerald-300 rounded text-[11px] font-semibold flex items-center gap-1 mx-auto transition-colors"
                        >
                          <Send class="w-3 h-3 text-emerald-700" />
                          <span>Kirim Mutasi</span>
                        </button>
                      {:else}
                        <span class="text-[11px] text-slate-400 italic">Lokal</span>
                      {/if}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Footer Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-t border-slate-200 flex justify-end">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 bg-slate-800 hover:bg-slate-900 text-white rounded-lg text-xs font-bold transition-colors"
        >
          Tutup (ESC)
        </button>
      </div>
    </div>
  </div>
{/if}
