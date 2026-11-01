<script lang="ts">
  import { X, CheckCircle, Boxes, AlertTriangle } from "@lucide/svelte";
  import type { Product, StockAdjustmentInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    products: Product[];
    onClose: () => void;
    onSubmit: (input: StockAdjustmentInput) => void;
  }

  let { isOpen = false, products = [], onClose, onSubmit }: Props = $props();

  let selectedProductId = $state<string>("");
  let actualStock = $state<number>(0);
  let reason = $state("Hasil Perhitungan Fisik Stok Opname");

  let selectedProduct = $derived(products.find((p) => p.id === selectedProductId));
  let systemStock = $derived(selectedProduct?.current_stock ?? 0);
  let variance = $derived(actualStock - systemStock);

  $effect(() => {
    if (isOpen && products.length > 0 && !selectedProductId) {
      selectedProductId = products[0].id;
      actualStock = products[0].current_stock;
    }
  });

  function handleProductSelect(id: string) {
    selectedProductId = id;
    const p = products.find((prod) => prod.id === id);
    if (p) {
      actualStock = p.current_stock;
    }
  }

  function handleSubmit() {
    if (!selectedProductId) return;
    onSubmit({
      product_id: selectedProductId,
      actual_stock: actualStock,
      reason: reason.trim() || "Penyesuaian Stok Opname",
    });
    onClose();
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-md overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Boxes class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">Stok Opname & Penyesuaian Fisik</h3>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form Body -->
      <div class="p-5 space-y-4 text-xs text-slate-700">
        <div>
          <label for="opname-product" class="block font-semibold mb-1">Pilih Produk yang Dihitung</label>
          <select
            id="opname-product"
            value={selectedProductId}
            onchange={(e) => handleProductSelect(e.currentTarget.value)}
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-semibold text-slate-900"
          >
            {#each products as p}
              <option value={p.id}>
                {p.name} ({p.sku}) - Stok Sistem: {p.current_stock} {p.base_unit_name}
              </option>
            {/each}
          </select>
        </div>

        <!-- Perbandingan Stok Sistem vs Fisik -->
        <div class="grid grid-cols-2 gap-3 p-3 bg-slate-50 border border-slate-200 rounded-lg">
          <div>
            <span class="text-slate-500 block text-[11px]">Catatan Sistem:</span>
            <div class="font-bold font-mono text-base text-slate-800 mt-0.5">
              {systemStock} {selectedProduct?.base_unit_name || "Unit"}
            </div>
          </div>
          <div>
            <span class="text-slate-500 block text-[11px]">Selisih Hitungan:</span>
            <div class="font-bold font-mono text-base mt-0.5 {variance > 0 ? 'text-emerald-700' : variance < 0 ? 'text-rose-600' : 'text-slate-600'}">
              {variance > 0 ? `+${variance}` : variance} ({variance === 0 ? "Sesuai" : variance > 0 ? "Lebih" : "Kurang"})
            </div>
          </div>
        </div>

        <div>
          <label for="opname-actual-stock" class="block font-semibold mb-1">Jumlah Fisik Sebenarnya (Satuan Dasar)</label>
          <input
            id="opname-actual-stock"
            type="number"
            min="0"
            bind:value={actualStock}
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-lg font-mono font-bold text-slate-900"
          />
        </div>

        <div>
          <label for="opname-reason" class="block font-semibold mb-1">Alasan Penyesuaian / Catatan</label>
          <input
            id="opname-reason"
            type="text"
            bind:value={reason}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg"
          />
        </div>
      </div>

      <!-- Footer -->
      <div class="px-5 py-3.5 bg-slate-50 border-t border-slate-200 flex justify-end gap-2">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 text-xs font-medium text-slate-700 bg-white border border-slate-300 hover:bg-slate-50 rounded-lg"
        >
          Batal
        </button>
        <button
          type="button"
          onclick={handleSubmit}
          class="px-5 py-2 text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 rounded-lg shadow-xs flex items-center gap-1.5"
        >
          <CheckCircle class="w-4 h-4" />
          <span>Simpan Penyesuaian Stok</span>
        </button>
      </div>
    </div>
  </div>
{/if}
