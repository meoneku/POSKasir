<script lang="ts">
  import { onMount } from "svelte";
  import { ShieldCheck, User, KeyRound, Lock, ArrowRight, Delete, Check, AlertCircle } from "@lucide/svelte";
  import type { User as UserType, UserSession } from "$lib/types";
  import { api } from "$lib/api";

  interface Props {
    isOpen: boolean;
    users: UserType[];
    onLoginSuccess: (session: UserSession) => void;
  }

  let { isOpen = true, users = [], onLoginSuccess }: Props = $props();

  let loginMode = $state<"pin" | "password">("pin");

  // State PIN Login
  let selectedUserId = $state<string | null>(null);
  let pinInput = $state<string>("");
  let pinError = $state<string | null>(null);
  let isPinSubmitting = $state<boolean>(false);

  // State Password Login
  let usernameInput = $state<string>("admin");
  let passwordInput = $state<string>("");
  let passwordError = $state<string | null>(null);
  let isPasswordSubmitting = $state<boolean>(false);

  // Auto-select user pertama jika ada
  $effect(() => {
    if (users.length > 0 && !selectedUserId) {
      const firstCashier = users.find((u) => u.role === "cashier" && u.is_active === 1) || users[0];
      selectedUserId = firstCashier.id;
    }
  });

  // Listener keyboard untuk PIN numpad
  onMount(() => {
    const handleKeydown = (e: KeyboardEvent) => {
      if (!isOpen) return;

      if (loginMode === "pin") {
        if (e.key >= "0" && e.key <= "9") {
          handlePinPress(e.key);
        } else if (e.key === "Backspace") {
          handlePinBackspace();
        } else if (e.key === "Enter") {
          handlePinSubmit();
        } else if (e.key === "Escape") {
          pinInput = "";
          pinError = null;
        }
      }
    };

    window.addEventListener("keydown", handleKeydown);
    return () => window.removeEventListener("keydown", handleKeydown);
  });

  function handlePinPress(num: string) {
    if (pinInput.length < 6) {
      pinInput += num;
      pinError = null;

      // Otomatis submit jika sudah 4 digit
      if (pinInput.length === 4) {
        setTimeout(() => {
          handlePinSubmit();
        }, 120);
      }
    }
  }

  function handlePinBackspace() {
    pinInput = pinInput.slice(0, -1);
    pinError = null;
  }

  function handlePinClear() {
    pinInput = "";
    pinError = null;
  }

  async function handlePinSubmit() {
    if (pinInput.length < 4) {
      pinError = "Masukkan minimal 4 digit PIN";
      return;
    }

    isPinSubmitting = true;
    pinError = null;

    try {
      const session = await api.loginWithPin(pinInput, selectedUserId || undefined);
      pinInput = "";
      onLoginSuccess(session);
    } catch (err: any) {
      pinError = err?.message || err || "PIN Kasir salah. Silakan coba lagi.";
      pinInput = "";
    } finally {
      isPinSubmitting = false;
    }
  }

  async function handlePasswordSubmit(e: SubmitEvent) {
    e.preventDefault();
    if (!usernameInput.trim() || !passwordInput.trim()) {
      passwordError = "Username dan password wajib diisi";
      return;
    }

    isPasswordSubmitting = true;
    passwordError = null;

    try {
      const session = await api.loginWithCredentials(usernameInput.trim(), passwordInput.trim());
      passwordInput = "";
      onLoginSuccess(session);
    } catch (err: any) {
      passwordError = err?.message || err || "Username atau password salah.";
    } finally {
      isPasswordSubmitting = false;
    }
  }
</script>

{#if isOpen}
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div
    role="dialog"
    aria-modal="true"
    class="fixed inset-0 z-50 bg-slate-950/85 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div class="bg-white border border-slate-300 rounded-2xl shadow-2xl w-full max-w-md overflow-hidden flex flex-col">
      <!-- Header Login -->
      <div class="px-6 pt-6 pb-4 bg-slate-50 border-b border-slate-200 text-center">
        <div class="w-12 h-12 rounded-xl bg-emerald-600 text-white mx-auto flex items-center justify-center shadow-sm mb-3">
          <ShieldCheck class="w-7 h-7" />
        </div>
        <h2 class="text-base font-bold text-slate-900">POS Kasir Desktop</h2>
        <p class="text-xs text-slate-500 mt-0.5">Silakan masuk untuk memulai operasional kasir</p>

        <!-- Tab Pemilihan Mode Login -->
        <div class="mt-4 grid grid-cols-2 p-1 bg-slate-200/80 rounded-lg text-xs font-semibold">
          <button
            type="button"
            onclick={() => {
              loginMode = "pin";
              pinError = null;
            }}
            class="py-1.5 rounded-md transition-colors flex items-center justify-center gap-1.5 {loginMode === 'pin' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
          >
            <KeyRound class="w-3.5 h-3.5" />
            <span>PIN Cepat (Kasir)</span>
          </button>
          <button
            type="button"
            onclick={() => {
              loginMode = "password";
              passwordError = null;
            }}
            class="py-1.5 rounded-md transition-colors flex items-center justify-center gap-1.5 {loginMode === 'password' ? 'bg-white text-emerald-800 shadow-xs' : 'text-slate-600 hover:text-slate-900'}"
          >
            <Lock class="w-3.5 h-3.5" />
            <span>Password (Admin)</span>
          </button>
        </div>
      </div>

      <!-- Body Form -->
      <div class="p-6">
        {#if loginMode === "pin"}
          <!-- MODE 1: LOGIN PIN CEPAT KASIR -->
          <div class="space-y-4">
            <!-- Pilihan Profil Kasir -->
            <div>
              <label for="cashier-select" class="block text-xs font-semibold text-slate-700 mb-1.5">
                Pilih Akun Kasir Bertugas:
              </label>
              <div class="grid grid-cols-2 gap-2 max-h-28 overflow-y-auto pr-1">
                {#each users.filter((u) => u.is_active === 1) as u}
                  <button
                    type="button"
                    onclick={() => {
                      selectedUserId = u.id;
                      pinInput = "";
                      pinError = null;
                    }}
                    class="p-2 rounded-lg border text-left flex items-center gap-2 transition-colors {selectedUserId === u.id ? 'border-emerald-600 bg-emerald-50 text-emerald-950 ring-1 ring-emerald-600 font-bold' : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50 font-medium'}"
                  >
                    <div class="w-7 h-7 rounded-full {selectedUserId === u.id ? 'bg-emerald-600 text-white' : 'bg-slate-100 text-slate-700'} flex items-center justify-center text-xs font-bold shrink-0">
                      {u.full_name.slice(0, 1).toUpperCase()}
                    </div>
                    <div class="truncate text-xs leading-tight">
                      <div class="truncate">{u.full_name}</div>
                      <div class="text-[10px] text-slate-500 font-normal capitalize">{u.role}</div>
                    </div>
                  </button>
                {/each}
              </div>
            </div>

            <!-- Display Indikator PIN -->
            <div class="pt-2 text-center">
              <div class="text-xs text-slate-600 font-medium mb-2">
                Masukkan PIN Kasir (Numpad / Keyboard)
              </div>
              <div class="flex justify-center items-center gap-3">
                {#each [0, 1, 2, 3] as idx}
                  <div
                    class="w-4 h-4 rounded-full border-2 transition-all {idx < pinInput.length ? 'bg-emerald-600 border-emerald-600 scale-110 shadow-xs' : 'border-slate-300 bg-slate-100'}"
                  ></div>
                {/each}
              </div>
            </div>

            <!-- Pesan Error PIN -->
            {#if pinError}
              <div class="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg flex items-center gap-2">
                <AlertCircle class="w-4 h-4 text-rose-600 shrink-0" />
                <span>{pinError}</span>
              </div>
            {/if}

            <!-- On-Screen Keypad Numpad -->
            <div class="grid grid-cols-3 gap-2 pt-1 max-w-xs mx-auto">
              {#each ["1", "2", "3", "4", "5", "6", "7", "8", "9"] as num}
                <button
                  type="button"
                  onclick={() => handlePinPress(num)}
                  class="h-11 rounded-lg bg-slate-100 hover:bg-slate-200 active:bg-slate-300 text-slate-900 font-mono font-bold text-base transition-colors border border-slate-200 shadow-xs"
                >
                  {num}
                </button>
              {/each}
              <button
                type="button"
                onclick={handlePinClear}
                class="h-11 rounded-lg bg-slate-100 hover:bg-slate-200 active:bg-slate-300 text-slate-600 font-semibold text-xs transition-colors border border-slate-200 flex items-center justify-center"
              >
                C
              </button>
              <button
                type="button"
                onclick={() => handlePinPress("0")}
                class="h-11 rounded-lg bg-slate-100 hover:bg-slate-200 active:bg-slate-300 text-slate-900 font-mono font-bold text-base transition-colors border border-slate-200 shadow-xs"
              >
                0
              </button>
              <button
                type="button"
                onclick={handlePinBackspace}
                class="h-11 rounded-lg bg-slate-100 hover:bg-slate-200 active:bg-slate-300 text-slate-600 transition-colors border border-slate-200 flex items-center justify-center"
              >
                <Delete class="w-4 h-4" />
              </button>
            </div>

            <!-- Tombol Konfirmasi Manual -->
            <button
              type="button"
              disabled={isPinSubmitting || pinInput.length < 4}
              onclick={handlePinSubmit}
              class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 text-white font-bold text-xs rounded-lg transition-colors flex items-center justify-center gap-1.5 shadow-sm"
            >
              <Check class="w-4 h-4" />
              <span>{isPinSubmitting ? 'Memeriksa PIN...' : 'Masuk Kasir'}</span>
            </button>
          </div>
        {:else}
          <!-- MODE 2: LOGIN USERNAME & PASSWORD (ADMIN) -->
          <form onsubmit={handlePasswordSubmit} class="space-y-4">
            <div>
              <label for="admin-login-username" class="block text-xs font-semibold text-slate-700 mb-1">
                Username / Nama Akun
              </label>
              <div class="relative">
                <input
                  id="admin-login-username"
                  type="text"
                  bind:value={usernameInput}
                  placeholder="admin"
                  required
                  class="w-full pl-9 pr-3 py-2 bg-white border border-slate-300 rounded-lg text-xs font-semibold text-slate-900 focus:outline-none focus:border-emerald-600 shadow-xs"
                />
                <User class="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
              </div>
            </div>

            <div>
              <label for="admin-login-password" class="block text-xs font-semibold text-slate-700 mb-1">
                Password
              </label>
              <div class="relative">
                <input
                  id="admin-login-password"
                  type="password"
                  bind:value={passwordInput}
                  placeholder="••••••••"
                  required
                  class="w-full pl-9 pr-3 py-2 bg-white border border-slate-300 rounded-lg text-xs font-semibold text-slate-900 focus:outline-none focus:border-emerald-600 shadow-xs"
                />
                <Lock class="w-4 h-4 text-slate-400 absolute left-3 top-2.5" />
              </div>
            </div>

            {#if passwordError}
              <div class="p-2.5 bg-rose-50 border border-rose-200 text-rose-800 text-xs rounded-lg flex items-center gap-2">
                <AlertCircle class="w-4 h-4 text-rose-600 shrink-0" />
                <span>{passwordError}</span>
              </div>
            {/if}

            <button
              type="submit"
              disabled={isPasswordSubmitting}
              class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 text-white font-bold text-xs rounded-lg transition-colors flex items-center justify-center gap-1.5 shadow-sm"
            >
              <ArrowRight class="w-4 h-4" />
              <span>{isPasswordSubmitting ? 'Memeriksa Kredensial...' : 'Masuk sebagai Administrator'}</span>
            </button>
          </form>
        {/if}
      </div>

      <!-- Catatan Kredensial Pengujian Default -->
      <div class="px-6 py-3 bg-slate-100 border-t border-slate-200 text-[11px] text-slate-500 flex justify-between items-center">
        <span>Akun Bawaan:</span>
        <span class="font-mono text-slate-700">
          Kasir (PIN: <strong class="text-emerald-700">1111</strong>) | Admin (PIN: <strong class="text-emerald-700">1234</strong> / pass: <strong class="text-emerald-700">admin123</strong>)
        </span>
      </div>
    </div>
  </div>
{/if}
