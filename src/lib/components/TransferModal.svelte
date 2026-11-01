<script lang="ts">
  import { X, Send, Plus, Trash2, ArrowRight, Boxes, Building2, AlertTriangle } from "@lucide/svelte";
  import type { Branch, Product, CreateInterBranchTransferInput, CreateTransferItemInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    currentBranch: Branch | null;
    branches: Branch[];
    products: Product[];
    onClose: () => void;
    onSubmit: (input: CreateInterBranchTransferInput) => void;
  }

  let {
    isOpen = false,
    currentBranch = null,
    branches = [],
    products = [],
    onClose,
    onSubmit,
  }: Props = $props();

  let destinationBranchId = $state("");
  let notes = $state("");
  let items = $state<
    {
      productId: string;
      unitId: string;
      quantity: number;
    }[]
  >([]);

  // Daftar cabang tujuan (mengecualikan cabang aktif saat ini)
  let targetBranches = $derived(
    branches.filter((b) => b.id !== currentBranch?.id)
  );

  $effect(() => {
    if (isOpen) {
      destinationBranchId = targetBranches[0]?.id || "";
      notes = "";
      if (products.length > 0) {
        items = [
          {
            productId: products[0].id,
            unitId: products[0].units[0]?.id || "",
            quantity: 1,
          },
        ];
      } else {
        items = [];
      }
    }
  });

  function addItem() {
    if (products.length > 0) {
      items = [
        ...items,
        {
          productId: products[0].id,
          unitId: products[0].units[0]?.id || "",
          quantity: 1,
        },
      ];
    }
  }

  function removeItem(index: number) {
    items = items.filter((_, i) => i !== index);
  }

  function handleProductChange(index: number, newProductId: string) {
    const prod = products.find((p) => p.id === newProductId);
    if (prod) {
      const updated = [...items];
      updated[index].productId = newProductId;
      updated[index].unitId = prod.units[0]?.id || "";
      updated[index].quantity = 1;
      items = updated;
    }
  }

  function handleUnitChange(index: number, newUnitId: string) {
    const updated = [...items];
    updated[index].unitId = newUnitId;
    items = updated;
  }

  function handleQuantityChange(index: number, val: number) {
    const updated = [...items];
    updated[index].quantity = Math.max(1, val);
    items = updated;
  }

  function getAvailableStock(productId: string): number {
    const prod = products.find((p) => p.id === productId);
    return prod ? prod.current_stock : 0;
  }

  function getBaseMultiplier(productId: string, unitId: string): number {
    const prod = products.find((p) => p.id === productId);
    const unit = prod?.units.find((u) => u.id === unitId);
    return unit ? unit.multiplier : 1;
  }

  function handleSubmit() {
    if (!destinationBranchId) return;
    if (items.length === 0) return;

    // Validasi stok cukup
    for (const item of items) {
      const stock = getAvailableStock(item.productId);
      const mult = getBaseMultiplier(item.productId, item.unitId);
      const totalBase = item.quantity * mult;
      if (totalBase > stock) {
        alert("Jumlah barang transfer melebihi sisa stok gudang cabang!");
        return;
      }
    }

    const payloadItems: CreateTransferItemInput[] = items.map((i) => ({
      product_id: i.productId,
      unit_id: i.unitId,
      quantity: i.quantity,
    }));

    onSubmit({
      destination_branch_id: destinationBranchId,
      notes: notes.trim() || undefined,
      items: payloadItems,
    });
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    }
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
    <div
      class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-2xl overflow-hidden flex flex-col max-h-[90vh]"
    >
      <!-- Header Modal -->
      <div class="px-5 py-4 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Send class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Kirim Mutasi Stok Antar Cabang</h3>
            <p class="text-xs text-slate-500">Kirim transfer stok barang ke toko cabang lain (Status: In Transit)</p>
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

      <div class="p-5 overflow-y-auto space-y-4 flex-1">
        <!-- Rute Transfer: Asal -> Tujuan -->
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 bg-slate-50 p-3.5 rounded-lg border border-slate-200">
          <div>
            <span class="text-[11px] font-semibold text-slate-500 uppercase tracking-wide block mb-1">
              Cabang Asal (Pengirim)
            </span>
            <div class="font-bold text-sm text-slate-900 flex items-center gap-1.5">
              <Building2 class="w-4 h-4 text-emerald-600" />
              <span>{currentBranch?.name || "Kantor Pusat"} ({currentBranch?.code || "PST"})</span>
            </div>
            <p class="text-[11px] text-slate-500 mt-0.5">{currentBranch?.address || "-"}</p>
          </div>

          <div>
            <label for="destination-branch" class="text-[11px] font-semibold text-slate-500 uppercase tracking-wide block mb-1">
              Cabang Tujuan (Penerima) *
            </label>
            <select
              id="destination-branch"
              bind:value={destinationBranchId}
              class="w-full text-xs bg-white border border-slate-300 rounded-lg px-3 py-2 font-semibold text-slate-800 focus:outline-none focus:ring-1 focus:ring-emerald-500"
            >
              {#each targetBranches as tb}
                <option value={tb.id}>
                  {tb.name} ({tb.code}) {tb.is_headquarters ? "[Pusat]" : ""}
                </option>
              {/each}
            </select>
          </div>
        </div>

        <!-- Daftar Barang yang Ditransfer -->
        <div class="space-y-2">
          <div class="flex justify-between items-center">
            <span class="text-xs font-bold text-slate-800 uppercase tracking-wide flex items-center gap-1.5">
              <Boxes class="w-4 h-4 text-emerald-600" />
              Rincian Barang yang Dimutasi ({items.length} Baris)
            </span>
            <button
              type="button"
              onclick={addItem}
              class="px-2.5 py-1 bg-emerald-50 hover:bg-emerald-100 text-emerald-800 border border-emerald-300 rounded text-xs font-semibold flex items-center gap-1 shadow-2xs"
            >
              <Plus class="w-3.5 h-3.5 text-emerald-700" />
              <span>Tambah Baris</span>
            </button>
          </div>

          <div class="space-y-2 max-h-60 overflow-y-auto pr-1">
            {#each items as item, idx}
              {@const selectedProd = products.find((p) => p.id === item.productId)}
              {@const currentStock = selectedProd ? selectedProd.current_stock : 0}
              {@const multiplier = getBaseMultiplier(item.productId, item.unitId)}
              {@const totalBaseNeeded = item.quantity * multiplier}
              {@const isExceeded = totalBaseNeeded > currentStock}

              <div class="p-3 bg-white border rounded-lg shadow-2xs space-y-2 {isExceeded ? 'border-rose-300 bg-rose-50/20' : 'border-slate-200'}">
                <div class="flex items-center gap-2">
                  <span class="w-5 text-center text-xs font-mono font-bold text-slate-400">#{idx + 1}</span>

                  <!-- Pilih Produk -->
                  <div class="flex-1">
                    <select
                      value={item.productId}
                      onchange={(e) => handleProductChange(idx, e.currentTarget.value)}
                      class="w-full text-xs bg-slate-50 border border-slate-300 rounded px-2.5 py-1.5 text-slate-900 font-medium"
                    >
                      {#each products as p}
                        <option value={p.id}>{p.name} (Stok: {p.current_stock} {p.base_unit_name})</option>
                      {/each}
                    </select>
                  </div>

                  <!-- Pilih Satuan -->
                  <div class="w-32">
                    <select
                      value={item.unitId}
                      onchange={(e) => handleUnitChange(idx, e.currentTarget.value)}
                      class="w-full text-xs bg-slate-50 border border-slate-300 rounded px-2.5 py-1.5 text-slate-900 font-medium"
                    >
                      {#if selectedProd}
                        {#each selectedProd.units as u}
                          <option value={u.id}>{u.unit_name} (x{u.multiplier})</option>
                        {/each}
                      {/if}
                    </select>
                  </div>

                  <!-- Kuantitas -->
                  <div class="w-20">
                    <input
                      type="number"
                      min="1"
                      value={item.quantity}
                      oninput={(e) => handleQuantityChange(idx, Number(e.currentTarget.value))}
                      class="w-full text-xs bg-slate-50 border border-slate-300 rounded px-2.5 py-1.5 text-slate-900 font-mono font-bold text-center"
                    />
                  </div>

                  <!-- Hapus Baris -->
                  <button
                    type="button"
                    onclick={() => removeItem(idx)}
                    disabled={items.length <= 1}
                    class="text-slate-300 hover:text-rose-600 disabled:opacity-30 p-1.5 transition-colors"
                    title="Hapus baris"
                  >
                    <Trash2 class="w-4 h-4" />
                  </button>
                </div>

                <!-- Info Stok & Peringatan Jika Melebihi -->
                <div class="flex justify-between items-center text-[11px] px-7">
                  <span class="text-slate-500">
                    Total fisik: <strong class="text-slate-800">{totalBaseNeeded} {selectedProd?.base_unit_name}</strong>
                    (Sisa gudang: {currentStock} {selectedProd?.base_unit_name})
                  </span>
                  {#if isExceeded}
                    <span class="text-rose-600 font-bold flex items-center gap-1">
                      <AlertTriangle class="w-3.5 h-3.5" />
                      Stok kurang {totalBaseNeeded - currentStock} {selectedProd?.base_unit_name}!
                    </span>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Catatan Pengiriman / Armada -->
        <div>
          <label for="transfer-notes" class="block text-xs font-semibold text-slate-700 mb-1">
            Catatan Transfer / No. Resi Ekspedisi:
          </label>
          <input
            id="transfer-notes"
            type="text"
            bind:value={notes}
            placeholder="misal: Diantar Armada Box Toko (Driver Pak Eko) / Resi J&T Cargo"
            class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-xs text-slate-900 focus:outline-none focus:ring-1 focus:ring-emerald-500"
          />
        </div>
      </div>

      <!-- Footer Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-t border-slate-200 flex justify-between items-center">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 bg-white hover:bg-slate-100 text-slate-700 border border-slate-300 rounded-lg text-xs font-bold transition-colors"
        >
          Batal (ESC)
        </button>

        <button
          type="button"
          onclick={handleSubmit}
          disabled={items.length === 0 || !destinationBranchId}
          class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white rounded-lg text-xs font-bold shadow-xs flex items-center gap-2 transition-all active:scale-95"
        >
          <Send class="w-4 h-4" />
          <span>Terbitkan Surat Mutasi & Kurangi Stok</span>
        </button>
      </div>
    </div>
  </div>
{/if}
