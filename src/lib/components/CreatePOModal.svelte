<script lang="ts">
  import { X, Plus, Trash2, CheckCircle, ReceiptText } from "@lucide/svelte";
  import type { Supplier, Product, CreatePurchaseInput, PurchaseItemInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    suppliers: Supplier[];
    products: Product[];
    onClose: () => void;
    onSubmit: (input: CreatePurchaseInput) => void;
  }

  let { isOpen = false, suppliers = [], products = [], onClose, onSubmit }: Props = $props();

  let selectedSupplierId = $state<string>("");
  let paymentType = $state<"cash" | "credit">("credit");
  let dueDate = $state<string>("");
  let isTaxIncluded = $state(true);

  interface TempPOItem {
    product_id: string;
    unit_id: string;
    multiplier: number;
    unit_name: string;
    quantity: number;
    unit_cost: number;
    subtotal: number;
  }

  let items = $state<TempPOItem[]>([]);

  // Otomatis isi due date berdasarkan ketentuan tempo supplier
  $effect(() => {
    if (selectedSupplierId && suppliers.length > 0) {
      const sup = suppliers.find((s) => s.id === selectedSupplierId);
      if (sup) {
        if (sup.payment_terms_days > 0) {
          paymentType = "credit";
          const d = new Date();
          d.setDate(d.getDate() + sup.payment_terms_days);
          dueDate = d.toISOString().split("T")[0];
        } else {
          paymentType = "cash";
          dueDate = "";
        }
      }
    }
  });

  // Default tambahkan 1 baris item kosong saat buka modal
  $effect(() => {
    if (isOpen && items.length === 0 && products.length > 0) {
      addItemRow();
    }
  });

  function addItemRow() {
    if (products.length === 0) return;
    const firstProd = products[0];
    const firstUnit = firstProd.units[0];
    items = [
      ...items,
      {
        product_id: firstProd.id,
        unit_id: firstUnit.id,
        multiplier: firstUnit.multiplier,
        unit_name: firstUnit.unit_name,
        quantity: 10,
        unit_cost: firstUnit.retail_price * 0.8, // Estimasi modal 80% dari retail
        subtotal: 10 * (firstUnit.retail_price * 0.8),
      },
    ];
  }

  function handleProductChange(index: number, newProdId: string) {
    const prod = products.find((p) => p.id === newProdId);
    if (!prod) return;
    const unit = prod.units[0];
    items[index].product_id = prod.id;
    items[index].unit_id = unit.id;
    items[index].multiplier = unit.multiplier;
    items[index].unit_name = unit.unit_name;
    items[index].unit_cost = unit.retail_price * 0.8;
    items[index].subtotal = items[index].quantity * items[index].unit_cost;
  }

  function handleUnitChange(index: number, newUnitId: string) {
    const prod = products.find((p) => p.id === items[index].product_id);
    if (!prod) return;
    const unit = prod.units.find((u) => u.id === newUnitId);
    if (!unit) return;
    items[index].unit_id = unit.id;
    items[index].multiplier = unit.multiplier;
    items[index].unit_name = unit.unit_name;
    items[index].unit_cost = unit.retail_price * 0.8;
    items[index].subtotal = items[index].quantity * items[index].unit_cost;
  }

  function handleQuantityOrCostChange(index: number) {
    items[index].subtotal = items[index].quantity * items[index].unit_cost;
  }

  function removeItemRow(index: number) {
    items = items.filter((_, i) => i !== index);
  }

  let subtotalAmount = $derived(items.reduce((sum, item) => sum + item.subtotal, 0));
  let taxAmount = $derived(isTaxIncluded ? subtotalAmount * 0.11 : 0);
  let totalAmount = $derived(subtotalAmount + taxAmount);

  function handleSubmit() {
    if (!selectedSupplierId || items.length === 0) return;

    onSubmit({
      supplier_id: selectedSupplierId,
      payment_type: paymentType,
      due_date: paymentType === "credit" ? dueDate : undefined,
      subtotal: subtotalAmount,
      tax_id: isTaxIncluded ? "t0000000-0000-0000-0000-000000000001" : undefined,
      tax_amount: taxAmount,
      total_amount: totalAmount,
      paid_amount: paymentType === "cash" ? totalAmount : 0,
      items: items.map((i) => ({
        product_id: i.product_id,
        unit_id: i.unit_id,
        multiplier: i.multiplier,
        quantity: i.quantity,
        unit_cost: i.unit_cost,
        subtotal: i.subtotal,
      })),
    });

    items = [];
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
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-3xl max-h-[90vh] overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <ReceiptText class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">Buat Pesanan Pembelian / Purchase Order (PO)</h3>
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
        <!-- Supplier & Ketentuan Bayar -->
        <div class="grid grid-cols-3 gap-3">
          <div>
            <label for="po-supplier" class="block font-semibold mb-1">Pilih Pemasok / Supplier *</label>
            <select
              id="po-supplier"
              bind:value={selectedSupplierId}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-semibold text-slate-900"
            >
              <option value="">-- Pilih Supplier --</option>
              {#each suppliers as s}
                <option value={s.id}>{s.name} ({s.payment_terms_days} hari tempo)</option>
              {/each}
            </select>
          </div>

          <div>
            <label for="po-payment-type" class="block font-semibold mb-1">Metode Pembayaran</label>
            <select
              id="po-payment-type"
              bind:value={paymentType}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-semibold"
            >
              <option value="credit">Tempo / Hutang Dagang (Credit)</option>
              <option value="cash">Tunai Langsung (Cash)</option>
            </select>
          </div>

          <div>
            <label for="po-due-date" class="block font-semibold mb-1">Jatuh Tempo Pembayaran</label>
            <input
              id="po-due-date"
              type="date"
              disabled={paymentType === "cash"}
              bind:value={dueDate}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono {paymentType === 'cash' ? 'bg-slate-100 text-slate-400' : ''}"
            />
          </div>
        </div>

        <!-- Tabel Item Pembelian -->
        <div class="space-y-2 pt-2 border-t border-slate-200">
          <div class="flex justify-between items-center">
            <span class="font-bold text-slate-900">Daftar Barang yang Dipesan</span>
            <button
              type="button"
              onclick={addItemRow}
              class="px-2.5 py-1 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300 font-semibold flex items-center gap-1"
            >
              <Plus class="w-3.5 h-3.5" />
              <span>Tambah Baris</span>
            </button>
          </div>

          <div class="border border-slate-200 rounded-lg overflow-hidden">
            <table class="w-full text-left text-xs">
              <thead class="bg-slate-50 border-b border-slate-200 text-slate-600 uppercase font-semibold">
                <tr>
                  <th class="py-2 px-3">Produk</th>
                  <th class="py-2 px-3">Satuan</th>
                  <th class="py-2 px-3 w-20">Kuantitas</th>
                  <th class="py-2 px-3 text-right">Harga Beli Satuan</th>
                  <th class="py-2 px-3 text-right">Subtotal</th>
                  <th class="py-2 px-2 text-center w-10"></th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-100">
                {#each items as item, idx}
                  {@const prod = products.find((p) => p.id === item.product_id)}
                  <tr>
                    <td class="py-2 px-3">
                      <select
                        value={item.product_id}
                        onchange={(e) => handleProductChange(idx, e.currentTarget.value)}
                        class="w-full bg-white border border-slate-300 rounded px-2 py-1"
                      >
                        {#each products as p}
                          <option value={p.id}>{p.name} ({p.sku})</option>
                        {/each}
                      </select>
                    </td>

                    <td class="py-2 px-3">
                      {#if prod}
                        <select
                          value={item.unit_id}
                          onchange={(e) => handleUnitChange(idx, e.currentTarget.value)}
                          class="w-full bg-white border border-slate-300 rounded px-2 py-1"
                        >
                          {#each prod.units as u}
                            <option value={u.id}>{u.unit_name} (x{u.multiplier})</option>
                          {/each}
                        </select>
                      {/if}
                    </td>

                    <td class="py-2 px-3">
                      <input
                        type="number"
                        min="1"
                        bind:value={item.quantity}
                        oninput={() => handleQuantityOrCostChange(idx)}
                        class="w-full bg-white border border-slate-300 rounded px-2 py-1 font-mono text-center font-bold"
                      />
                    </td>

                    <td class="py-2 px-3 text-right">
                      <input
                        type="number"
                        min="0"
                        bind:value={item.unit_cost}
                        oninput={() => handleQuantityOrCostChange(idx)}
                        class="w-32 bg-white border border-slate-300 rounded px-2 py-1 font-mono text-right font-semibold"
                      />
                    </td>

                    <td class="py-2 px-3 text-right font-mono font-bold text-slate-900">
                      Rp {item.subtotal.toLocaleString("id-ID")}
                    </td>

                    <td class="py-2 px-2 text-center">
                      <button
                        type="button"
                        onclick={() => removeItemRow(idx)}
                        class="text-slate-300 hover:text-rose-600 p-1"
                      >
                        <Trash2 class="w-3.5 h-3.5" />
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>

        <!-- Ringkasan Nilai & Pajak Masukan -->
        <div class="bg-slate-50 border border-slate-200 rounded-lg p-3 space-y-1.5 text-xs">
          <div class="flex justify-between">
            <span>Subtotal Pembelian:</span>
            <span class="font-semibold text-slate-800">Rp {subtotalAmount.toLocaleString("id-ID")}</span>
          </div>

          <div class="flex justify-between items-center pt-1 border-t border-slate-200">
            <label class="flex items-center gap-1.5 cursor-pointer">
              <input
                type="checkbox"
                bind:checked={isTaxIncluded}
                class="text-emerald-600 rounded"
              />
              <span>Catat Pajak Masukan (PPN 11%)</span>
            </label>
            <span class="font-mono text-slate-700">Rp {taxAmount.toLocaleString("id-ID")}</span>
          </div>

          <div class="flex justify-between items-baseline pt-2 border-t border-slate-300 text-slate-900">
            <span class="font-bold text-sm">TOTAL NILAI PO:</span>
            <span class="font-bold text-base font-mono text-emerald-700">
              Rp {totalAmount.toLocaleString("id-ID")}
            </span>
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
          disabled={!selectedSupplierId || items.length === 0}
          onclick={handleSubmit}
          class="px-5 py-2 text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 rounded-lg shadow-xs flex items-center gap-1.5"
        >
          <CheckCircle class="w-4 h-4" />
          <span>Terbitkan Purchase Order</span>
        </button>
      </div>
    </div>
  </div>
{/if}
