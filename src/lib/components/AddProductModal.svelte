<script lang="ts">
  import { X, Plus, Trash2, CheckCircle, PackagePlus, AlertCircle } from "@lucide/svelte";
  import type { Category, CreateProductFullInput, UnitInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    categories: Category[];
    onClose: () => void;
    onSubmit: (input: CreateProductFullInput) => void;
  }

  let { isOpen = false, categories = [], onClose, onSubmit }: Props = $props();

  let sku = $state("");
  let barcode = $state("");
  let name = $state("");
  let categoryId = $state<string | undefined>(undefined);
  let minStockAlert = $state(5);
  let isTaxable = $state(1);
  let baseUnitName = $state("Pcs");
  let initialStock = $state(0);

  let units = $state<UnitInput[]>([
    {
      unit_name: "Pcs",
      multiplier: 1,
      barcode: undefined,
      retail_price: 10000,
      wholesale_price_1: 9500,
      wholesale_price_2: 9000,
      min_qty_wholesale_1: 6,
      min_qty_wholesale_2: 24,
    },
  ]);

  function addUnitRow() {
    units = [
      ...units,
      {
        unit_name: "Dus",
        multiplier: 12,
        barcode: undefined,
        retail_price: 115000,
        wholesale_price_1: 110000,
        wholesale_price_2: 105000,
        min_qty_wholesale_1: 2,
        min_qty_wholesale_2: 5,
      },
    ];
  }

  function removeUnitRow(index: number) {
    if (units.length <= 1) return;
    units = units.filter((_, i) => i !== index);
  }

  function handleSubmit() {
    if (!sku.trim() || !name.trim()) return;
    onSubmit({
      sku: sku.trim().toUpperCase(),
      barcode: barcode.trim() || undefined,
      name: name.trim(),
      category_id: categoryId || undefined,
      min_stock_alert: minStockAlert,
      is_taxable: isTaxable,
      base_unit_name: units[0].unit_name,
      initial_stock: initialStock,
      units: units.map((u, i) => ({
        ...u,
        multiplier: i === 0 ? 1 : u.multiplier,
      })),
    });
    resetForm();
  }

  function resetForm() {
    sku = "";
    barcode = "";
    name = "";
    initialStock = 0;
    units = [
      {
        unit_name: "Pcs",
        multiplier: 1,
        barcode: undefined,
        retail_price: 10000,
        wholesale_price_1: 9500,
        wholesale_price_2: 9000,
        min_qty_wholesale_1: 6,
        min_qty_wholesale_2: 24,
      },
    ];
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
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <PackagePlus class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">Tambah Produk Baru & Multi-Satuan</h3>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Body Form -->
      <div class="p-5 overflow-y-auto space-y-4 text-xs text-slate-700">
        <!-- Informasi Dasar -->
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="prod-sku" class="block font-semibold mb-1">Kode SKU / Kode Barang *</label>
            <input
              id="prod-sku"
              type="text"
              placeholder="misal: MINYAK-02"
              bind:value={sku}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono uppercase font-bold"
            />
          </div>
          <div>
            <label for="prod-barcode" class="block font-semibold mb-1">Barcode Scanner (EAN-13 / UPC)</label>
            <input
              id="prod-barcode"
              type="text"
              placeholder="Scan barcode di sini..."
              bind:value={barcode}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono"
            />
          </div>
        </div>

        <div>
          <label for="prod-name" class="block font-semibold mb-1">Nama Produk Lengkap *</label>
          <input
            id="prod-name"
            type="text"
            placeholder="misal: Minyak Goreng Bimoli Spesial 2 Liter"
            bind:value={name}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-semibold"
          />
        </div>

        <div class="grid grid-cols-3 gap-3">
          <div>
            <label for="prod-cat" class="block font-semibold mb-1">Kategori Produk</label>
            <select
              id="prod-cat"
              bind:value={categoryId}
              class="w-full px-2.5 py-1.5 bg-white border border-slate-300 rounded-lg"
            >
              <option value={undefined}>-- Pilih Kategori --</option>
              {#each categories as c}
                <option value={c.id}>{c.name}</option>
              {/each}
            </select>
          </div>
          <div>
            <label for="prod-stock" class="block font-semibold mb-1">Stok Awal (Satuan Dasar)</label>
            <input
              id="prod-stock"
              type="number"
              bind:value={initialStock}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono"
            />
          </div>
          <div>
            <label for="prod-min-alert" class="block font-semibold mb-1">Batas Minimum Peringatan</label>
            <input
              id="prod-min-alert"
              type="number"
              bind:value={minStockAlert}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono"
            />
          </div>
        </div>

        <!-- Aturan Pajak -->
        <div class="flex items-center gap-3 p-2.5 bg-slate-50 border border-slate-200 rounded-lg">
          <span class="font-semibold">Status Perpajakan:</span>
          <label class="flex items-center gap-1.5 cursor-pointer">
            <input
              type="radio"
              name="prod_tax"
              value={1}
              checked={isTaxable === 1}
              onchange={() => (isTaxable = 1)}
              class="text-emerald-600"
            />
            <span>BKP (Kena Pajak PPN 11%)</span>
          </label>
          <label class="flex items-center gap-1.5 cursor-pointer">
            <input
              type="radio"
              name="prod_tax"
              value={0}
              checked={isTaxable === 0}
              onchange={() => (isTaxable = 0)}
              class="text-emerald-600"
            />
            <span>Non-BKP (Bebas Pajak)</span>
          </label>
        </div>

        <!-- Hierarki Multi-Satuan & Matrix Harga -->
        <div class="space-y-2 pt-2 border-t border-slate-200">
          <div class="flex justify-between items-center">
            <div>
              <h4 class="font-bold text-slate-900 text-xs">Konversi Multi-Satuan & Matrix Harga Tier</h4>
              <p class="text-[11px] text-slate-500">Satuan pertama adalah satuan dasar terkecil (multiplier = 1)</p>
            </div>
            <button
              type="button"
              onclick={addUnitRow}
              class="px-2.5 py-1 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300 font-semibold flex items-center gap-1"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>Tambah Satuan Kemasan</span>
            </button>
          </div>

          <div class="space-y-2">
            {#each units as u, idx}
              <div class="p-3 bg-slate-50 border border-slate-200 rounded-lg space-y-2">
                <div class="flex justify-between items-center">
                  <span class="font-bold text-emerald-800">
                    Satuan #{idx + 1} {idx === 0 ? '(Satuan Dasar / Unit Terkecil)' : ''}
                  </span>
                  {#if idx > 0}
                    <button
                      type="button"
                      onclick={() => removeUnitRow(idx)}
                      class="text-rose-500 hover:text-rose-700 p-0.5"
                      title="Hapus Satuan"
                    >
                      <Trash2 class="w-3.5 h-3.5" />
                    </button>
                  {/if}
                </div>

                <div class="grid grid-cols-4 gap-2">
                  <div>
                    <span class="block text-[10px] text-slate-500 mb-0.5">Nama Satuan (Pcs/Pak/Dus)</span>
                    <input
                      type="text"
                      bind:value={u.unit_name}
                      class="w-full px-2 py-1 bg-white border border-slate-300 rounded font-semibold"
                    />
                  </div>
                  <div>
                    <span class="block text-[10px] text-slate-500 mb-0.5">Pengali (Isi dalam Dasar)</span>
                    <input
                      type="number"
                      disabled={idx === 0}
                      bind:value={u.multiplier}
                      class="w-full px-2 py-1 bg-white border border-slate-300 rounded font-mono {idx === 0 ? 'bg-slate-100 text-slate-400' : ''}"
                    />
                  </div>
                  <div>
                    <span class="block text-[10px] text-slate-500 mb-0.5">Harga Eceran (Rp)</span>
                    <input
                      type="number"
                      bind:value={u.retail_price}
                      class="w-full px-2 py-1 bg-white border border-slate-300 rounded font-mono font-bold text-slate-900"
                    />
                  </div>
                  <div>
                    <span class="block text-[10px] text-slate-500 mb-0.5">Grosir Tier 2 / Dus (Rp)</span>
                    <input
                      type="number"
                      bind:value={u.wholesale_price_2}
                      class="w-full px-2 py-1 bg-white border border-slate-300 rounded font-mono text-emerald-700 font-bold"
                    />
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Footer Modal -->
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
          <span>Simpan Produk & Satuan</span>
        </button>
      </div>
    </div>
  </div>
{/if}
