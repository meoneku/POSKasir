<script lang="ts">
  import { X, User, Lock, KeyRound, ShieldCheck, Check } from "@lucide/svelte";
  import type { User as UserType, CreateUserAccountInput, UpdateUserAccountInput } from "$lib/types";

  interface Props {
    isOpen: boolean;
    userToEdit: UserType | null;
    onClose: () => void;
    onSave: (data: CreateUserAccountInput | UpdateUserAccountInput) => void;
  }

  let { isOpen = false, userToEdit = null, onClose, onSave }: Props = $props();

  let fullName = $state("");
  let username = $state("");
  let role = $state<"cashier" | "supervisor" | "admin">("cashier");
  let password = $state("");
  let pinCode = $state("");
  let isActive = $state(1);

  $effect(() => {
    if (isOpen) {
      if (userToEdit) {
        fullName = userToEdit.full_name;
        username = userToEdit.username;
        role = (userToEdit.role as any) || "cashier";
        pinCode = userToEdit.pin_code || "";
        password = "";
        isActive = userToEdit.is_active;
      } else {
        fullName = "";
        username = "";
        role = "cashier";
        password = "";
        pinCode = "1234";
        isActive = 1;
      }
    }
  });

  function handleSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!fullName.trim() || !username.trim()) return;

    if (userToEdit) {
      const payload: UpdateUserAccountInput = {
        id: userToEdit.id,
        full_name: fullName.trim(),
        username: username.trim(),
        role,
        password: password.trim() ? password.trim() : undefined,
        pin_code: pinCode.trim() ? pinCode.trim() : undefined,
        is_active: isActive,
      };
      onSave(payload);
    } else {
      const payload: CreateUserAccountInput = {
        full_name: fullName.trim(),
        username: username.trim(),
        role,
        password: password.trim() ? password.trim() : "123456",
        pin_code: pinCode.trim() ? pinCode.trim() : "1234",
      };
      onSave(payload);
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    class="fixed inset-0 z-50 bg-slate-900/60 backdrop-blur-xs flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-200 rounded-xl shadow-xl w-full max-w-md overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="px-5 py-3.5 bg-slate-50 border-b border-slate-200 flex justify-between items-center">
        <div class="flex items-center gap-2">
          <ShieldCheck class="w-5 h-5 text-emerald-600" />
          <h3 class="font-bold text-slate-900 text-sm">
            {userToEdit ? "Edit Akun Pengguna / Kasir" : "Tambah Akun Kasir / Pengguna Baru"}
          </h3>
        </div>
        <button
          onclick={onClose}
          class="p-1 rounded text-slate-400 hover:text-slate-700 hover:bg-slate-200 transition-colors"
        >
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Form Body -->
      <form onsubmit={handleSubmit} class="p-5 space-y-3.5 text-xs">
        <div>
          <label for="user-full-name" class="block font-semibold text-slate-700 mb-1">
            Nama Lengkap Kasir / Operator *
          </label>
          <input
            id="user-full-name"
            type="text"
            bind:value={fullName}
            required
            placeholder="Contoh: Dewi Sartika"
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 font-semibold focus:outline-none focus:border-emerald-600 shadow-xs"
          />
        </div>

        <div>
          <label for="user-username" class="block font-semibold text-slate-700 mb-1">
            Username (Untuk Login Admin/Sistem) *
          </label>
          <input
            id="user-username"
            type="text"
            bind:value={username}
            required
            placeholder="Contoh: kasir_dewi"
            class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 font-mono focus:outline-none focus:border-emerald-600 shadow-xs"
          />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label for="user-role" class="block font-semibold text-slate-700 mb-1">
              Peran / Level Hak Akses *
            </label>
            <select
              id="user-role"
              bind:value={role}
              class="w-full px-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 font-medium focus:outline-none focus:border-emerald-600 shadow-xs"
            >
              <option value="cashier">Kasir (Cashier)</option>
              <option value="supervisor">Supervisor Toko</option>
              <option value="admin">Superadmin / Pemilik</option>
            </select>
          </div>

          <div>
            <label for="user-pin" class="block font-semibold text-slate-700 mb-1">
              PIN Kasir (4–6 Digit) *
            </label>
            <div class="relative">
              <input
                id="user-pin"
                type="text"
                maxlength="6"
                bind:value={pinCode}
                placeholder="1234"
                required
                class="w-full pl-8 pr-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 font-mono font-bold focus:outline-none focus:border-emerald-600 shadow-xs"
              />
              <KeyRound class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-2.5" />
            </div>
          </div>
        </div>

        <div>
          <label for="user-password" class="block font-semibold text-slate-700 mb-1 flex items-center justify-between">
            <span>Password (Untuk Login Admin)</span>
            {#if userToEdit}
              <span class="text-[10px] text-slate-400 font-normal">Kosongkan jika tidak diubah</span>
            {/if}
          </label>
          <div class="relative">
            <input
              id="user-password"
              type="password"
              bind:value={password}
              placeholder={userToEdit ? "••••••••" : "Default: 123456"}
              class="w-full pl-8 pr-3 py-2 bg-white border border-slate-300 rounded-lg text-slate-900 focus:outline-none focus:border-emerald-600 shadow-xs"
            />
            <Lock class="w-3.5 h-3.5 text-slate-400 absolute left-2.5 top-2.5" />
          </div>
        </div>

        {#if userToEdit}
          <div class="pt-2 border-t border-slate-200">
            <label class="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={isActive === 1}
                onchange={(e) => (isActive = e.currentTarget.checked ? 1 : 0)}
                class="rounded border-slate-300 text-emerald-600 focus:ring-emerald-500"
              />
              <span class="font-semibold text-slate-800">Akun Aktif (Dapat Login)</span>
            </label>
          </div>
        {/if}

        <div class="flex justify-end gap-2 pt-3 border-t border-slate-200">
          <button
            type="button"
            onclick={onClose}
            class="px-3 py-2 rounded-lg border border-slate-300 text-slate-700 font-semibold hover:bg-slate-100 transition-colors"
          >
            Batal
          </button>
          <button
            type="submit"
            class="px-4 py-2 rounded-lg bg-emerald-600 hover:bg-emerald-700 text-white font-bold transition-colors flex items-center gap-1.5 shadow-sm"
          >
            <Check class="w-4 h-4" />
            <span>{userToEdit ? "Simpan Perubahan" : "Simpan Pengguna"}</span>
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
