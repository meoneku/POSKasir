<script lang="ts">
  import { X, Hash, Plus, Minus, Check } from "@lucide/svelte";
  import type { CartItem } from "$lib/types";

  interface Props {
    isOpen: boolean;
    item: CartItem | null;
    onClose: () => void;
    onSaveQty: (newQty: number) => void;
  }

  let { isOpen = false, item = null, onClose, onSaveQty }: Props = $props();

  let qtyInput = $state(1);
  let inputRef = $state<HTMLInputElement | null>(null);

  $effect(() => {
    if (isOpen && item) {
      qtyInput = item.quantity;
      setTimeout(() => {
        if (inputRef) {
          inputRef.focus();
          inputRef.select();
        }
      }, 50);
    }
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      onClose();
    } else if (e.key === "Enter") {
      e.preventDefault();
      handleConfirm();
    }
  }

  function handleConfirm() {
    const val = Number(qtyInput);
    if (val > 0) {
      onSaveQty(val);
    }
  }

  function handleNumpad(digit: string) {
    if (digit === "C") {
      qtyInput = 0;
    } else {
      const current = qtyInput === 0 ? "" : String(qtyInput);
      qtyInput = Number(current + digit);
    }
    inputRef?.focus();
  }

  function adjust(delta: number) {
    const next = Math.max(1, qtyInput + delta);
    qtyInput = next;
    inputRef?.focus();
  }
</script>

{#if isOpen && item}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onkeydown={handleKeydown}
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-sm overflow-hidden flex flex-col">
      <!-- Header Modal -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Hash class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Ubah Jumlah Kuantitas (F4)</h3>
            <p class="text-[11px] text-slate-500 line-clamp-1">{item.product.name} ({item.selected_unit.unit_name})</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Input Angka Besar & Numpad -->
      <div class="p-4 space-y-3">
        <!-- Input Utama -->
        <div>
          <label for="edit-qty-input" class="block text-xs font-semibold text-slate-700 mb-1">
            Kuantitas ({item.selected_unit.unit_name})
          </label>
          <div class="flex items-center gap-1.5">
            <button
              type="button"
              onclick={() => adjust(-1)}
              class="w-11 h-11 rounded-lg bg-slate-100 hover:bg-slate-200 border border-slate-300 flex items-center justify-center text-slate-800 font-bold active:scale-95"
            >
              <Minus class="w-5 h-5" />
            </button>
            <input
              id="edit-qty-input"
              bind:this={inputRef}
              type="number"
              min="1"
              bind:value={qtyInput}
              class="flex-1 h-11 px-3 bg-white border-2 border-emerald-600 rounded-lg text-center font-mono font-bold text-xl text-slate-900 focus:outline-none"
            />
            <button
              type="button"
              onclick={() => adjust(1)}
              class="w-11 h-11 rounded-lg bg-slate-100 hover:bg-slate-200 border border-slate-300 flex items-center justify-center text-slate-800 font-bold active:scale-95"
            >
              <Plus class="w-5 h-5" />
            </button>
          </div>
        </div>

        <!-- Tombol Tambah Cepat -->
        <div class="grid grid-cols-4 gap-1.5 text-xs font-semibold">
          <button
            type="button"
            onclick={() => adjust(5)}
            class="py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300"
          >
            +5
          </button>
          <button
            type="button"
            onclick={() => adjust(10)}
            class="py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300"
          >
            +10
          </button>
          <button
            type="button"
            onclick={() => adjust(24)}
            class="py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300"
          >
            +24
          </button>
          <button
            type="button"
            onclick={() => (qtyInput = 1)}
            class="py-1.5 bg-slate-100 hover:bg-slate-200 text-slate-700 rounded border border-slate-300"
          >
            Reset (1)
          </button>
        </div>

        <!-- Touch Numpad Grid -->
        <div class="grid grid-cols-3 gap-1.5 pt-2 border-t border-slate-200 text-sm font-bold font-mono">
          {#each ["1", "2", "3", "4", "5", "6", "7", "8", "9", "C", "0", "00"] as d}
            <button
              type="button"
              onclick={() => handleNumpad(d)}
              class="py-2.5 rounded-lg border border-slate-200 transition-colors active:scale-95 {d === 'C'
                ? 'bg-rose-50 text-rose-700 hover:bg-rose-100'
                : 'bg-slate-50 text-slate-800 hover:bg-slate-100'}"
            >
              {d}
            </button>
          {/each}
        </div>

        <!-- Estimasi Subtotal -->
        <div class="p-2.5 bg-slate-50 border border-slate-200 rounded-lg flex justify-between items-center text-xs">
          <span class="text-slate-500">Estimasi Subtotal:</span>
          <span class="font-bold font-mono text-emerald-700 text-sm">
            Rp {(qtyInput * item.unit_price).toLocaleString("id-ID")}
          </span>
        </div>
      </div>

      <!-- Action Footer -->
      <div class="px-5 py-3 bg-slate-50 border-t border-slate-200 flex justify-end gap-2">
        <button
          type="button"
          onclick={onClose}
          class="px-4 py-2 bg-white hover:bg-slate-100 text-slate-700 font-medium text-xs rounded-lg transition-colors border border-slate-300"
        >
          Batal (ESC)
        </button>
        <button
          type="button"
          onclick={handleConfirm}
          class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-lg transition-colors flex items-center gap-1.5 shadow-xs"
        >
          <Check class="w-4 h-4" />
          <span>Simpan Jumlah (Enter)</span>
        </button>
      </div>
    </div>
  </div>
{/if}
