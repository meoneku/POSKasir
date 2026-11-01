<script lang="ts">
  import { X, Building2, Plus, Check } from "@lucide/svelte";
  import type { CreateBranchInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (input: CreateBranchInput) => void;
  }

  let { isOpen = false, onClose, onSubmit }: Props = $props();

  let code = $state("");
  let name = $state("");
  let address = $state("");
  let phone = $state("");
  let isHeadquarters = $state(false);
  let apiEndpoint = $state("");
  let apiKey = $state("");

  $effect(() => {
    if (isOpen) {
      code = "";
      name = "";
      address = "";
      phone = "";
      isHeadquarters = false;
      apiEndpoint = "http://100.64.0.4:8080";
      apiKey = `KEY-BR-${Math.floor(1000 + Math.random() * 9000)}`;
    }
  });

  function handleSubmit() {
    if (!code.trim() || !name.trim()) return;
    onSubmit({
      code: code.trim().toUpperCase(),
      name: name.trim(),
      address: address.trim() || undefined,
      phone: phone.trim() || undefined,
      is_headquarters: isHeadquarters ? 1 : 0,
      api_endpoint: apiEndpoint.trim() || undefined,
      api_key: apiKey.trim() || undefined,
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
      class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-lg overflow-hidden flex flex-col max-h-[90vh]"
    >
      <!-- Header Modal -->
      <div class="px-5 py-4 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <Building2 class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Tambah Toko / Cabang Baru</h3>
            <p class="text-xs text-slate-500">Daftarkan titik cabang baru ke dalam jaringan multi-cabang terpadu</p>
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

      <div class="p-5 space-y-3.5 overflow-y-auto flex-1 text-xs">
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="branch-code" class="block font-semibold text-slate-700 mb-1">
              Kode Cabang *
            </label>
            <input
              id="branch-code"
              type="text"
              bind:value={code}
              placeholder="misal: SMG-04"
              class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 font-mono uppercase focus:outline-none focus:ring-1 focus:ring-emerald-500"
            />
          </div>

          <div>
            <label for="branch-phone" class="block font-semibold text-slate-700 mb-1">
              No. Telepon / Hotline
            </label>
            <input
              id="branch-phone"
              type="text"
              bind:value={phone}
              placeholder="misal: 024-8889911"
              class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 font-mono focus:outline-none focus:ring-1 focus:ring-emerald-500"
            />
          </div>
        </div>

        <div>
          <label for="branch-name" class="block font-semibold text-slate-700 mb-1">
            Nama Cabang / Toko *
          </label>
          <input
            id="branch-name"
            type="text"
            bind:value={name}
            placeholder="misal: Cabang Semarang Simpang Lima"
            class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 font-medium focus:outline-none focus:ring-1 focus:ring-emerald-500"
          />
        </div>

        <div>
          <label for="branch-address" class="block font-semibold text-slate-700 mb-1">
            Alamat Lengkap Cabang
          </label>
          <textarea
            id="branch-address"
            rows="2"
            bind:value={address}
            placeholder="misal: Jl. Pandanaran No. 50, Semarang"
            class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 focus:outline-none focus:ring-1 focus:ring-emerald-500"
          ></textarea>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="branch-api-endpoint" class="block font-semibold text-slate-700 mb-1">
              Alamat Sinkronisasi (IP Mesh VPN / URL)
            </label>
            <input
              id="branch-api-endpoint"
              type="text"
              bind:value={apiEndpoint}
              placeholder="http://100.64.0.4:8080"
              class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 font-mono focus:outline-none focus:ring-1 focus:ring-emerald-500"
            />
          </div>

          <div>
            <label for="branch-api-key" class="block font-semibold text-slate-700 mb-1">
              Kunci API / Token Otorisasi
            </label>
            <input
              id="branch-api-key"
              type="text"
              bind:value={apiKey}
              class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg text-slate-900 font-mono focus:outline-none focus:ring-1 focus:ring-emerald-500"
            />
          </div>
        </div>

        <!-- Checkbox Kantor Pusat -->
        <div class="pt-2">
          <label class="flex items-center gap-2 cursor-pointer select-none">
            <input
              type="checkbox"
              bind:checked={isHeadquarters}
              class="w-4 h-4 rounded text-emerald-600 border-slate-300 focus:ring-emerald-500"
            />
            <span class="font-semibold text-slate-800">Tentukan sebagai Kantor Pusat (HQ Central Hub)</span>
          </label>
          <p class="text-[11px] text-slate-500 ml-6 mt-0.5">
            Kantor pusat berkedudukan sebagai master katalog & pusat konsolidasi sinkronisasi
          </p>
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
          disabled={!code.trim() || !name.trim()}
          class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white rounded-lg text-xs font-bold shadow-xs flex items-center gap-2 transition-all active:scale-95"
        >
          <Plus class="w-4 h-4" />
          <span>Simpan Cabang Baru</span>
        </button>
      </div>
    </div>
  </div>
{/if}
