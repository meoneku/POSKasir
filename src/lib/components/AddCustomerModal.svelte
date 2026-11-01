<script lang="ts">
  import { X, UserPlus, Check, Building2, Phone, MapPin, CreditCard, Shield } from "@lucide/svelte";
  import type { CreateCustomerInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (input: CreateCustomerInput) => void;
  }

  let { isOpen = false, onClose, onSubmit }: Props = $props();

  let code = $state("");
  let name = $state("");
  let npwpNik = $state("");
  let phone = $state("");
  let address = $state("");
  let customerType = $state<"retail" | "wholesale" | "vip">("wholesale");
  let creditLimit = $state(10000000);

  $effect(() => {
    if (isOpen) {
      const randNum = Math.floor(100 + Math.random() * 900);
      code = `CUST-${randNum}`;
      name = "";
      npwpNik = "";
      phone = "";
      address = "";
      customerType = "wholesale";
      creditLimit = 10000000;
    }
  });

  function handleSubmit() {
    if (!name.trim()) return;
    onSubmit({
      code: code.trim(),
      name: name.trim(),
      npwp_nik: npwpNik.trim() || undefined,
      phone: phone.trim() || undefined,
      address: address.trim() || undefined,
      customer_type: customerType,
      credit_limit: Number(creditLimit) || 0,
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
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-md overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <UserPlus class="w-5 h-5 text-emerald-600" />
          <div>
            <h3 class="font-bold text-slate-900 text-sm">Tambah Pelanggan / Mitra Grosir Baru</h3>
            <p class="text-[11px] text-slate-500">Daftarkan mitra toko rekanan untuk plafon piutang tempo</p>
          </div>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form Inputs -->
      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="p-5 space-y-3.5 text-xs text-slate-700">
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="cust-code" class="block font-semibold mb-1 text-slate-800">Kode Pelanggan</label>
            <input
              id="cust-code"
              type="text"
              bind:value={code}
              required
              class="w-full px-3 py-2 bg-slate-50 border border-slate-300 rounded-lg font-mono font-bold focus:bg-white focus:border-emerald-600 focus:outline-none"
            />
          </div>
          <div>
            <label for="cust-type" class="block font-semibold mb-1 text-slate-800">Tipe Pelanggan</label>
            <select
              id="cust-type"
              bind:value={customerType}
              class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-semibold focus:border-emerald-600 focus:outline-none"
            >
              <option value="wholesale">Grosir (B2B)</option>
              <option value="retail">Retail (Eceran)</option>
              <option value="vip">Mitra VIP Khusus</option>
            </select>
          </div>
        </div>

        <div>
          <label for="cust-name" class="block font-semibold mb-1 text-slate-800">Nama Toko / Nama Pelanggan *</label>
          <input
            id="cust-name"
            type="text"
            placeholder="misal: Toko Berkah Mandiri / Pak Haji Ahmad"
            bind:value={name}
            required
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-bold text-slate-900 focus:border-emerald-600 focus:outline-none"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="cust-phone" class="block font-semibold mb-1 text-slate-800">Nomor Telepon / WhatsApp</label>
            <input
              id="cust-phone"
              type="text"
              placeholder="0812-xxxx-xxxx"
              bind:value={phone}
              class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg focus:border-emerald-600 focus:outline-none"
            />
          </div>
          <div>
            <label for="cust-npwp" class="block font-semibold mb-1 text-slate-800">NPWP / NIK (Faktur Pajak)</label>
            <input
              id="cust-npwp"
              type="text"
              placeholder="01.xxx.xxx.x-xxx.000"
              bind:value={npwpNik}
              class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-mono focus:border-emerald-600 focus:outline-none"
            />
          </div>
        </div>

        <div>
          <label for="cust-address" class="block font-semibold mb-1 text-slate-800">Alamat Pengiriman / Alamat Usaha</label>
          <textarea
            id="cust-address"
            rows="2"
            placeholder="Jalan, Nomor Toko, Pasar / Wilayah Pengiriman..."
            bind:value={address}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg focus:border-emerald-600 focus:outline-none"
          ></textarea>
        </div>

        <div class="pt-2 border-t border-slate-200">
          <label for="cust-credit-limit" class="block font-semibold mb-1 text-slate-800 flex items-center gap-1.5">
            <Shield class="w-3.5 h-3.5 text-emerald-600" />
            <span>Plafon Kredit Maksimum (Credit Limit Rp)</span>
          </label>
          <input
            id="cust-credit-limit"
            type="number"
            min="0"
            step="100000"
            bind:value={creditLimit}
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg font-mono font-bold text-emerald-800 focus:border-emerald-600 focus:outline-none"
          />
          <p class="text-[11px] text-slate-500 mt-1">Batas maksimum piutang tempo yang diizinkan sebelum transaksi diblokir sistem.</p>
        </div>

        <!-- Footer Buttons -->
        <div class="flex justify-end gap-2 pt-3 border-t border-slate-200">
          <button
            type="button"
            onclick={onClose}
            class="px-4 py-2 bg-white hover:bg-slate-100 border border-slate-300 rounded-lg font-medium transition-colors text-slate-700"
          >
            Batal (ESC)
          </button>
          <button
            type="submit"
            class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg font-bold transition-colors flex items-center gap-1.5 shadow-xs"
          >
            <Check class="w-4 h-4" />
            <span>Simpan Pelanggan</span>
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
