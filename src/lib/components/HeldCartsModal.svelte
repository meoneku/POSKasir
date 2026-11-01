<script lang="ts">
  import { X, PauseCircle, PlayCircle, Trash2, ShoppingCart, Clock } from "@lucide/svelte";
  import type { CartItem } from "$lib/types";

  interface HeldCart {
    id: string;
    name: string;
    items: CartItem[];
    time: string;
  }

  interface Props {
    isOpen: boolean;
    heldCarts: HeldCart[];
    onClose: () => void;
    onRecall: (cartId: string) => void;
    onDelete: (cartId: string) => void;
  }

  let {
    isOpen = false,
    heldCarts = [],
    onClose,
    onRecall,
    onDelete,
  }: Props = $props();

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
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col max-h-[85vh]">
      <!-- Header Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <PauseCircle class="w-5 h-5 text-amber-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Daftar Transaksi Tertahan (Held Carts)</h3>
            <p class="text-[11px] text-slate-500">Buka kembali nota yang ditunda saat pelanggan menambah barang</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Konten Daftar Held Carts -->
      <div class="p-4 flex-1 overflow-y-auto space-y-3">
        {#if heldCarts.length === 0}
          <div class="py-12 flex flex-col items-center justify-center text-slate-400 text-xs">
            <ShoppingCart class="w-10 h-10 mb-2 stroke-1 text-slate-300" />
            <p class="font-medium text-slate-600">Tidak ada nota yang sedang ditahan</p>
            <p class="text-[11px] text-slate-400 mt-1">Tekan F8 di layar kasir untuk menahan transaksi aktif</p>
          </div>
        {:else}
          {#each heldCarts as hc}
            {@const totalItems = hc.items.reduce((sum, it) => sum + it.quantity, 0)}
            {@const subtotal = hc.items.reduce((sum, it) => sum + it.subtotal, 0)}
            <div class="bg-slate-50 border border-slate-200 rounded-lg p-3 hover:border-emerald-600 transition-all flex flex-col gap-2">
              <div class="flex justify-between items-start">
                <div>
                  <h4 class="font-bold text-slate-900 text-xs flex items-center gap-1.5">
                    <span>{hc.name}</span>
                  </h4>
                  <div class="flex items-center gap-2 text-[11px] text-slate-500 mt-0.5">
                    <span class="flex items-center gap-1">
                      <Clock class="w-3 h-3 text-slate-400" />
                      <span>{hc.time}</span>
                    </span>
                    <span>&bull;</span>
                    <span>{totalItems} Item</span>
                  </div>
                </div>

                <div class="text-right">
                  <div class="font-bold font-mono text-emerald-700 text-xs">
                    Rp {subtotal.toLocaleString("id-ID")}
                  </div>
                </div>
              </div>

              <!-- Preview Item Ringkas -->
              <div class="bg-white border border-slate-200 rounded p-2 text-[11px] text-slate-700 max-h-24 overflow-y-auto divide-y divide-slate-100">
                {#each hc.items as it}
                  <div class="py-1 flex justify-between items-center first:pt-0 last:pb-0">
                    <span class="line-clamp-1">{it.product.name} ({it.selected_unit.unit_name})</span>
                    <span class="font-mono text-slate-500 font-semibold">{it.quantity}x</span>
                  </div>
                {/each}
              </div>

              <!-- Action Buttons -->
              <div class="flex justify-end gap-2 pt-1 border-t border-slate-200">
                <button
                  type="button"
                  onclick={() => onDelete(hc.id)}
                  class="px-2.5 py-1 text-xs text-rose-600 hover:text-rose-700 hover:bg-rose-50 rounded font-semibold transition-colors flex items-center gap-1"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                  <span>Hapus</span>
                </button>
                <button
                  type="button"
                  onclick={() => onRecall(hc.id)}
                  class="px-3 py-1 bg-emerald-600 hover:bg-emerald-700 text-white rounded text-xs font-bold transition-colors flex items-center gap-1 shadow-xs"
                >
                  <PlayCircle class="w-3.5 h-3.5" />
                  <span>Muat Nota Ini</span>
                </button>
              </div>
            </div>
          {/each}
        {/if}
      </div>

      <!-- Footer Modal -->
      <div class="px-5 py-3 bg-slate-50 border-t border-slate-200 flex justify-between items-center">
        <span class="text-xs text-slate-500">Total Nota Ditahan: <strong class="text-slate-800">{heldCarts.length}</strong></span>
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-1.5 bg-slate-200 hover:bg-slate-300 text-slate-800 text-xs font-semibold rounded-lg transition-colors border border-slate-300"
        >
          Tutup (ESC)
        </button>
      </div>
    </div>
  </div>
{/if}
