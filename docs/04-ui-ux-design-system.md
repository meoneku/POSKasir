# 04. Sistem Desain UI/UX & Standar Interaksi

Dokumen ini memuat standar tampilan visual antarmuka dan pedoman perilaku UX yang dirancang agar **nyaman di mata kasir yang bekerja berjam-jam, cepat merespons, dan terprediksi**.

---

## 1. Aturan Baku Visual (No Purple, No Gradients)

1. ❌ **DILARANG menggunakan warna ungu / violet / magenta.**
2. ❌ **DILARANG menggunakan gradasi warna (No Gradients).**
   - Semua elemen menggunakan warna solid (*flat design*) dengan batas garis halus (*subtle 1px border*) dan bayangan lembut (*micro shadow*).
3. ✔️ **Palet Warna Utama: Emerald Green & Slate/Zinc**
   - Warna hijau zamrud (Emerald) adalah warna standar internasional untuk perdagangan, kasir, dan alur pembayaran positif.

---

## 2. Token Warna Solid

| Peran Token | Light Mode Hex | Dark Mode Hex | Fungsi Penggunaan |
| :--- | :--- | :--- | :--- |
| **Canvas / Background** | `#f8fafc` (Slate 50) | `#09090b` (Zinc 950) | Latar belakang dasar aplikasi. |
| **Card / Surface** | `#ffffff` (White) | `#18181b` (Zinc 900) | Kontainer kartu produk, keranjang, dan dialog modal. |
| **Borders & Dividers** | `#e2e8f0` (Slate 200) | `#27272a` (Zinc 800) | Garis pemisah 1px yang presisi dan tegas. |
| **Primary Brand (Emerald)** | `#059669` (Emerald 600) | `#10b981` (Emerald 500) | Tombol aksi utama (Bayar, Simpan, Indikator Aktif). |
| **Text Primary** | `#0f172a` (Slate 900) | `#fafafa` (Zinc 50) | Teks utama dengan kontras tajam. |
| **Text Muted** | `#64748b` (Slate 500) | `#a1a1aa` (Zinc 400) | Label sekunder, SKU, waktu. |
| **Success State** | `#16a34a` (Green 600) | `#22c55e` (Green 500) | Transaksi lunas, stok aman. |
| **Warning State** | `#d97706` (Amber 600) | `#f59e0b` (Amber 500) | Stok menipis, piutang mendekati plafon. |
| **Danger State** | `#dc2626` (Red 600) | `#ef4444` (Red 500) | Hapus item, void nota, jatuh tempo lewat. |
| **Info / Secondary Action**| `#2563eb` (Blue 600) | `#3b82f6` (Blue 500) | Cetak ulang, lihat faktur, detail pelanggan. |

---

## 3. Standar Perilaku UX (Natural & Desktop-Ergonomic)

### A. Autofokus & Interaksi Cepat
- **Kolom Pembayaran**: Saat popup dialog pembayaran dibuka, kursor otomatis masuk ke kolom input nominal uang, dan teks nominal default langsung ter-seleksi (*highlighted*). Kasir dapat langsung mengetik nominal uang yang diterima pembeli tanpa perlu menekan *backspace* atau mengklik mouse.
- **Deteksi Barcode Global**: Input dari scanner USB HID ditangkap oleh listener global. Kasir dapat langsung men-scan barang dari layar kasir manapun tanpa harus mengklik kolom pencarian terlebih dahulu.
- **Shortcut Keyboard Konsisten**:
  - `F2`: Fokus ke pencarian produk.
  - `F4`: Edit kuantitas baris item terpilih.
  - `F8`: Tahan (*Hold*) transaksi sementara.
  - `F9`: Ambil kembali (*Recall*) transaksi tertahan.
  - `F12`: Buka pembayaran dan selesaikan transaksi.
  - `ESC`: Tutup modal atau batalkan pilihan.

### B. Umpan Balik Tanpa Delay
- Setiap tombol memberikan feedback visual instan saat ditekan (`active:scale-[0.98]`) tanpa animasi berlebihan yang memperlambat alur kerja kasir.
- Notifikasi (*Toast*) muncul di sudut kanan atas dengan durasi 2 detik untuk status sukses dan tidak menghalangi area kerja utama.

---

## 4. Transisi Halaman & Komponen Skeleton Loader

Untuk menghilangkan layar berkedip kosong (*blank screen*) dan mencegah pergeseran tata letak (*layout shift*) saat berpindah halaman:
1. **Transisi Pudar Cepat (150ms)**:
   - Menggunakan transisi CSS *opacity fade-in* berdurasi 150 milidetik yang halus dan responsif.
2. **Skeleton Loader Terpadu**:
   - Diterapkan pada grid produk, tabel data transaksi/stok, dan kartu indikator KPI.

```html
<!-- Skeleton Grid Produk untuk Svelte 5 -->
{#if isLoading}
  <div class="grid grid-cols-2 md:grid-cols-4 gap-3 p-4">
    {#each Array(8) as _}
      <div class="bg-white dark:bg-zinc-900 border border-slate-200 dark:border-zinc-800 rounded-lg p-3 animate-pulse">
        <div class="h-28 bg-slate-200 dark:bg-zinc-800 rounded-md mb-2"></div>
        <div class="h-4 bg-slate-200 dark:bg-zinc-800 rounded w-3/4 mb-1"></div>
        <div class="h-4 bg-slate-200 dark:bg-zinc-800 rounded w-1/2"></div>
      </div>
    {/each}
  </div>
{:else}
  <!-- Daftar Produk Nyata -->
{/if}
```
