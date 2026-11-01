<script lang="ts">
  import { X, CheckCircle, Building } from "@lucide/svelte";
  import type { CreateSupplierInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (input: CreateSupplierInput) => void;
  }

  let { isOpen = false, onClose, onSubmit }: Props = $props();

  let name = $state("");
  let companyName = $state("");
  let npwp = $state("");
  let phone = $state("");
  let address = $state("");
  let paymentTermsDays = $state(14);

  function handleSubmit() {
    if (!name.trim()) return;
    onSubmit({
      name: name.trim(),
      company_name: companyName.trim() || undefined,
      npwp: npwp.trim() || undefined,
      phone: phone.trim() || undefined,
      address: address.trim() || undefined,
      payment_terms_days: paymentTermsDays,
    });
    reset();
  }

  function reset() {
    name = "";
    companyName = "";
    npwp = "";
    phone = "";
    address = "";
    paymentTermsDays = 14;
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
          <Building class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">Tambah Data Pemasok / Supplier Baru</h3>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form Body -->
      <div class="p-5 space-y-3 text-xs text-slate-700">
        <div>
          <label for="sup-name" class="block font-semibold mb-1">Nama Kontak Supplier *</label>
          <input
            id="sup-name"
            type="text"
            placeholder="misal: Bapak Anton / Sales Area"
            bind:value={name}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-semibold"
          />
        </div>

        <div>
          <label for="sup-company" class="block font-semibold mb-1">Nama Perusahaan / Distributor</label>
          <input
            id="sup-company"
            type="text"
            placeholder="misal: PT. Sinar Niaga Sejahtera"
            bind:value={companyName}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg"
          />
        </div>

        <div class="grid grid-cols-2 gap-2">
          <div>
            <label for="sup-phone" class="block font-semibold mb-1">Nomor Telepon / WA</label>
            <input
              id="sup-phone"
              type="text"
              placeholder="08123456789"
              bind:value={phone}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono"
            />
          </div>
          <div>
            <label for="sup-npwp" class="block font-semibold mb-1">NPWP Supplier</label>
            <input
              id="sup-npwp"
              type="text"
              placeholder="01.234.567..."
              bind:value={npwp}
              class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-mono"
            />
          </div>
        </div>

        <div>
          <label for="sup-terms" class="block font-semibold mb-1">Ketentuan Tempo Pembayaran (Hari)</label>
          <select
            id="sup-terms"
            bind:value={paymentTermsDays}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg font-semibold"
          >
            <option value={0}>0 Hari (COD / Tunai Langsung Saat Barang Datang)</option>
            <option value={7}>7 Hari (Tempo 1 Minggu)</option>
            <option value={14}>14 Hari (Tempo 2 Minggu)</option>
            <option value={30}>30 Hari (Tempo 1 Bulan)</option>
            <option value={60}>60 Hari (Tempo 2 Bulan)</option>
          </select>
        </div>

        <div>
          <label for="sup-address" class="block font-semibold mb-1">Alamat Gudang / Kantor</label>
          <textarea
            id="sup-address"
            rows="2"
            placeholder="Alamat lengkap supplier..."
            bind:value={address}
            class="w-full px-3 py-1.5 bg-white border border-slate-300 rounded-lg"
          ></textarea>
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
          disabled={!name.trim()}
          onclick={handleSubmit}
          class="px-5 py-2 text-xs font-bold text-white bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 rounded-lg shadow-xs flex items-center gap-1.5"
        >
          <CheckCircle class="w-4 h-4" />
          <span>Simpan Supplier</span>
        </button>
      </div>
    </div>
  </div>
{/if}
