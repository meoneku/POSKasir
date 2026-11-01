# 08 — Sistem Otentikasi, Multi-Level User & Hak Akses (RBAC)

Dokumen ini menjelaskan arsitektur keamanan, model otentikasi multi-level, pemisahan hak akses (*Role-Based Access Control*), dan alur operasional kasir harian pada repositori **POS Kasir Desktop**.

---

## 1. Latar Belakang & Filosofi Desain

Aplikasi kasir desktop dioperasikan di lantai toko (*store floor*) dengan karakteristik unik:
1. **Kecepatan di Meja Kasir (*Cashier Ergonomics*)**:
   - Meminta kasir mengetik username dan password panjang setiap pergantian shift atau setelah jeda istirahat memperlambat layanan dan antrean pelanggan.
   - Oleh karena itu, kasir utama menggunakan **PIN 4–6 Digit (Numpad-Friendly)** yang dapat ditekan dalam waktu kurang dari 2 detik.
2. **Pemisahan Wewenang (*Separation of Duties*)**:
   - Kasir biasa dilarang melihat laporan laba bersih, mengubah konfigurasi pajak, mengakses mutasi server lokal, atau mengubah data user lain.
   - Pemilik Toko / Superadmin memiliki kendali penuh atas manajemen pengguna, pengaturan finansial toko, master data supplier, dan otorisasi sistem.
3. **Audit Trail yang Jelas**:
   - Setiap transaksi penjualan, mutasi kas laci, dan pembukaan shift terikat ke identitas user yang sedang aktif (*session user*).

---

## 2. Struktur Level Pengguna (Roles)

Basis data SQLite mendukung 3 tingkatan peran utama pada tabel `users`:

### A. Superadmin / Administrator (`role = 'admin'`)
- Memiliki akses 100% tanpa batas ke seluruh modul aplikasi.
- **Wewenang Khusus**:
  - Menambah kasir baru, mengubah peran, mereset password / PIN kasir, dan menonaktifkan akun kasir yang berhenti bekerja.
  - Mengubah profil toko, logo, format nomor faktur, dan formula pajak (PPN 11%/12%).
  - Mengelola Purchase Order (PO) supplier, pelunasan hutang, dan stok opname.
  - Membuka laporan keuangan toko (omzet, laba kotor/bersih, rekapitulasi shift).
  - Mengaktifkan dan mengonfigurasi Embedded Axum Local Server & Cloud Sync.

### B. Kasir (`role = 'cashier'`)
- Pengguna harian di garda depan (*frontliner*) transaksi.
- **Wewenang**:
  - Layar Kasir Retail Cepat (`F2` hingga `F12`) & Kasir Grosir B2B.
  - Buka Shift Kasir (input modal awal) & Tutup Shift Kasir (hitung fisik laci kas).
  - Tambah mutasi kas masuk/keluar (*cash in / cash out*) untuk operasional kasir.
  - Cetak ulang struk thermal dan cetak Surat Jalan pengiriman.
  - Melihat katalog produk & cek ketersediaan stok fisik.
- **Batasan (Terkunci)**:
  - Menu Pengaturan Toko & Pajak **disembunyikan**.
  - Menu Manajemen Pengguna **disembunyikan**.
  - Menu Pembelian Supplier (PO & Hutang) **disembunyikan**.
  - Laporan laba/rugi keseluruhan **disembunyikan** (hanya melihat ringkasan shift miliknya).

### C. Supervisor / Manajer Toko (`role = 'supervisor'`)
- Pengawas operasional toko.
- **Wewenang**:
  - Seluruh wewenang Kasir.
  - Otorisasi pembatalan nota (*void*) dan diskon khusus.
  - Mengelola stok opname dan mutasi barang antar cabang.
  - Melihat laporan shift seluruh kasir.

---

## 3. Matriks Hak Akses Modul (RBAC Matrix)

| Modul & Fitur | Superadmin (`admin`) | Supervisor (`supervisor`) | Kasir (`cashier`) |
| :--- | :---: | :---: | :---: |
| **Kasir Retail Cepat** | ✔️ Akses Penuh | ✔️ Akses Penuh | ✔️ **Akses Penuh** |
| **Kasir Grosir B2B** | ✔️ Akses Penuh | ✔️ Akses Penuh | ✔️ **Akses Penuh** |
| **Buka / Tutup Shift Sendiri** | ✔️ Akses Penuh | ✔️ Akses Penuh | ✔️ **Akses Penuh** |
| **Kas Laci (+/-)** | ✔️ Akses Penuh | ✔️ Akses Penuh | ✔️ **Akses Penuh** |
| **Cari & Cek Stok Katalog** | ✔️ Akses Penuh | ✔️ Akses Penuh | ✔️ **Akses (Read-Only)** |
| **Tambah / Edit Produk** | ✔️ Akses Penuh | ✔️ Akses Penuh | ❌ Terkunci |
| **Stok Opname / Penyesuaian** | ✔️ Akses Penuh | ✔️ Akses Penuh | ❌ Terkunci |
| **Pemasok, PO & Hutang** | ✔️ Akses Penuh | 👁️ Read-Only | ❌ Terkunci |
| **Multi-Cabang & Mutasi Stok** | ✔️ Akses Penuh | ✔️ Akses Penuh | 👁️ Cek Stok Lintas Cabang |
| **Laporan Omzet & Penjualan** | ✔️ Akses Penuh | 👁️ Ringkasan Shift | ❌ Terkunci |
| **Pengaturan Toko & Pajak** | ✔️ Akses Penuh | ❌ Terkunci | ❌ Terkunci |
| **Manajemen Akun Pengguna** | ✔️ **Akses Penuh** | ❌ Terkunci | ❌ Terkunci |

---

## 4. Mekanisme Otentikasi & Alur Pengguna

### A. Dua Metode Login
1. **Mode PIN Cepat (Khusus Kasir)**:
   - Kasir memilih profilnya di layar login atau langsung mengetik **PIN 4–6 Digit**.
   - Input keypad numpad responsif di layar sentuh maupun keyboard fisik (`0-9`, `Enter` untuk konfirmasi, `Backspace` untuk hapus).
   - Mempercepat antrean kasir tanpa perlu mengetik teks panjang.
2. **Mode Kredensial Lengkap (Superadmin & Kasir)**:
   - Input **Username & Password**.
   - Digunakan oleh Superadmin untuk login pertama kali atau mengakses modul sensitif.

### B. Fitur Kunci Layar (*Lock Screen*)
- Tombol ikon gembok di header atas aplikasi memungkinkan kasir mengunci layar aplikasi seketika saat meninggalkan meja kasir (misalnya saat istirahat sholat atau makan).
- Transaksi keranjang dan shift aktif tetap terjaga aman di background.
- Untuk membuka kembali, kasir cukup mengetikkan PIN akunnya.

### C. Alur Sesi & Shift Kasir
- Saat Kasir berhasil login:
  - Jika belum ada shift kasir aktif atas nama kasir tersebut, sistem **langsung memunculkan dialog Buka Shift** meminta input modal uang receh awal laci.
  - Setelah shift dibuka, kasir langsung dialihkan ke layar POS Kasir Retail siap scan barcode.
- Saat kasir melakukan **Tutup Shift**, tersedia opsi:
  - "Tutup Shift & Tetap Login"
  - "Tutup Shift & Keluar (Logout)"

---

## 5. Skema Basis Data & Keamanan

### Tabel `users`
```sql
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    full_name TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK(role IN ('admin', 'supervisor', 'cashier', 'warehouse')),
    pin_code TEXT,
    is_active INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Keamanan Password & PIN
- Password dan PIN disimpan dengan enkripsi / hashing aman (SHA-256 + salt atau hashing terproteksi).
- API backend tidak pernah mengembalikan kolom `password_hash` atau `pin_code` ke antarmuka frontend (hanya mengembalikan model `UserSession` yang aman).
- Jika akun pengguna berstatus `is_active = 0`, pengguna tersebut langsung ditolak saat mencoba login.
