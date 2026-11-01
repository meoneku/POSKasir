# POS Kasir Desktop 🛒
### Solusi Pintar Kasir Retail, Grosir & Multi-Cabang

**POS Kasir Desktop** adalah aplikasi kasir terpadu yang dirancang khusus untuk mempermudah operasional toko, minimarket, distributor, dan bisnis grosir. Mulai dari pengadaan barang dari supplier, pengelolaan stok gudang, transaksi kasir eceran yang cepat, penjualan partai besar dengan faktur tempo, hingga sinkronisasi data antar cabang.

Aplikasi ini dapat langsung digunakan **tanpa perlu koneksi internet terus-menerus** dan **tanpa instalasi server database yang rumit**.

---

## 🌟 Keunggulan Utama

- ⚡ **Sangat Ringan & Cepat**: Dibuka dalam sekejap, tidak membebani komputer toko, dan transaksi kasir selesai dalam hitungan detik.
- 🔌 **Tetap Jalan Saat Internet Padam (Offline-First)**: Kasir tetap dapat melayani antrean pelanggan tanpa hambatan meskipun koneksi internet terputus.
- 🖨️ **Dukungan Printer Lengkap**: Cetak struk thermal kasir (ukuran 58mm & 80mm), cetak Faktur Pajak A4, dan cetak Surat Jalan Pengiriman barang.
- 📦 **Multi-Satuan & Harga Bertingkat**: Satu produk bisa dijual dalam berbagai satuan (misal: Beli per Dus dari supplier, lalu dijual per Dus, per Pak, atau per Pcs) dengan harga grosir otomatis.
- 🏢 **Siap untuk Toko Multi-Cabang**: Pantau ketersediaan stok di cabang lain dan kirim mutasi barang antar toko dengan mudah.
- 🔐 **Keamanan & Multi-Level Akses (Superadmin & Kasir)**: Login cepat kasir via PIN 4 digit numpad, proteksi menu sensitif (*RBAC*), kunci layar sementara (*lock screen*), dan manajemen akun kasir.
- 🛡️ **Data Tersimpan Mandiri**: Seluruh riwayat penjualan, data pelanggan, dan data keuangan tersimpan aman di komputer toko Anda sendiri.

---

## 🚀 Alur Bisnis Lengkap (Hulu ke Hilir)

Aplikasi ini mencakup seluruh siklus operasional usaha dagang Anda:

```
[1. Pemasok/Supplier] ➡️ [2. Gudang & Stok] ➡️ [3. Kasir Retail & Grosir] ➡️ [4. Multi-Cabang & Laporan]
```

### 1. Manajemen Pemasok & Pengadaan (Hulu)
- **Purchase Order (PO)**: Terbitkan surat pesanan resmi ke supplier lengkap dengan tanggal jatuh tempo pembayaran.
- **Penerimaan Barang**: Verifikasi barang yang tiba dari supplier; stok gudang otomatis bertambah seketika.
- **Hutang Usaha (AP)**: Pantau tagihan supplier yang belum lunas dan catat riwayat pelunasan secara bertahap.

### 2. Inventaris, Multi-Satuan & Stok Opname (Gudang)
- **Katalog Produk Rapi**: Simpan SKU, barcode produk, kategori, dan aturan Barang Kena Pajak (BKP).
- **Konversi Satuan Bertingkat**: Otomatis hitung isi kemasan (contoh: 1 Dus = 6 Pak = 24 Pcs) tanpa perlu menginput produk ganda.
- **Tingkatan Harga Grosir (Price Tiering)**: Tentukan harga eceran, harga grosir tier 1 (&ge;6 item), dan tier 2 (&ge;24 item / kartonan).
- **Stok Opname Fisik**: Sesuaikan selisih stok gudang dengan cepat saat penghitungan berkala.

### 3. Layar Kasir Penjualan Cepat (Hilir Retail)
- **Scanner Barcode Otomatis**: Cukup scan barcode dengan scanner USB/Wireless, barang langsung masuk ke keranjang belanja.
- **Shortcut Keyboard Lengkap**: Kasir dapat mengoperasikan transaksi hanya menggunakan keyboard:
  - `F2`: Cari produk / fokus barcode
  - `F4`: Ubah kuantitas barang
  - `F8`: Tahan nota belanja (*Hold Cart*)
  - `F9`: Buka kembali nota tertahan (*Recall Cart*)
  - `F12`: Buka pembayaran & selesaikan transaksi
  - `ESC`: Batal / tutup jendela dialog
- **Struk Belanja & Buka Laci Kasir**: Otomatis cetak nota kasir rapi dan buka laci kas (*cash drawer*) saat transaksi tunai selesai.
- **Manajemen Shift Kasir**: Pantau modal awal kasir, uang masuk/keluar, dan cetak laporan tutup shift (Laporan X / Z) untuk mencegah kebocoran dana kasir.

### 4. Penjualan Partai Besar & Grosir B2B
- **Plafon Kredit Mitra (Credit Limit)**: Batasi batas piutang maksimal per pelanggan grosir. Sistem otomatis memberi peringatan jika piutang pelanggan telah melampaui batas plafon.
- **Penjualan Piutang Tempo**: Opsi pembayaran kredit dengan jatuh tempo 7, 14, 30, atau 45 hari serta penerimaan Uang Muka (DP).
- **Faktur Penjualan & Pajak Standar A4**: Lengkap dengan identitas pembeli, NPWP/NIK, rincian PPN 11%, kalimat **Terbilang Rupiah**, dan kotak tanda tangan serah terima.
- **Surat Jalan / Delivery Order (DO)**: Dokumen pengiriman barang resmi tanpa mencantumkan nominal rupiah untuk menjaga privasi harga saat barang diantar kurir.

### 5. Multi-Cabang & Sinkronisasi Antar Toko
- **Mutasi Stok Antar Cabang**: Kirim stok barang dari satu cabang ke cabang lain lengkap dengan dokumen Surat Jalan Mutasi resmi.
- **Cek Stok Cabang Lain**: Periksa sisa stok barang di cabang kota lain secara langsung tanpa harus menelepon petugas gudang cabang.
- **Replikasi Dua Arah**: Sinkronkan data penjualan cabang ke kantor pusat melalui internet, atau hubungkan antar kasir di toko yang sama menggunakan jaringan Wi-Fi lokal.

### 6. Pengaturan Profil & Perpajakan
- **Kustomisasi Profil Toko**: Ubah nama toko, alamat, logo, nomor telepon, dan catatan kaki (*footer*) struk belanja.
- **Mesin Perpajakan Fleksibel**: Pilih perhitungan PPN 11% / 12% (Harga Sudah Termasuk Pajak / Pajak Ditambahkan Terpisah) serta pembedaan barang kena pajak vs barang bebas pajak.

---

## 🖥️ Kompatibilitas Sistem Operasi

Aplikasi ini tersedia untuk seluruh sistem operasi desktop:

| Platform | Format File | Keterangan |
| :--- | :--- | :--- |
| **Windows** | Installer `.exe` (NSIS) | Kompatibel dengan Windows 10 dan Windows 11 (64-bit). Dilengkapi wizard pemasangan dan shortcut desktop. |
| **Linux** | `.deb`, `.rpm`, `AppImage` | Siap dipasang di Ubuntu, Debian, Linux Mint, Fedora, Red Hat, dan distro Linux lainnya. Tersedia juga paket untuk Arch Linux. |
| **macOS** | `.dmg` | Mendukung Mac dengan prosesor Intel maupun Apple Silicon (M1, M2, M3, M4). |

---

## 💡 Panduan Cepat Penggunaan Harian

1. **Memulai Hari (Buka Toko)**:
   - Buka aplikasi POS Kasir Desktop.
   - Klik tombol **Buka Shift Kasir** di pojok kanan atas, lalu masukkan nominal uang modal awal kasir.
2. **Melayani Pembeli Eceran**:
   - Scan barcode barang atau ketik nama barang di kolom pencarian.
   - Tekan `F12` untuk pembayaran, pilih metode bayar (Tunai, QRIS, Transfer, atau Debit), masukkan nominal uang, lalu tekan Enter.
   - Struk otomatis tercetak dan laci uang kasir otomatis terbuka.
3. **Melayani Pelanggan Grosir**:
   - Masuk ke tab **Kasir Grosir**, pilih mitra toko yang terdaftar.
   - Masukkan barang pesanan dalam jumlah kartonan/dus.
   - Pilih pembayaran **Piutang Tempo** jika pelanggan membayar secara tempo, lalu tentukan tanggal jatuh tempo.
   - Cetak Faktur Penjualan A4 dan Surat Jalan untuk pengiriman.
4. **Mengakhiri Hari (Tutup Toko)**:
   - Klik tombol **Shift Kasir Aktif**, pilih **Tutup Shift Kasir**.
   - Masukkan total uang tunai fisik yang ada di laci kas.
   - Sistem akan mencocokkan uang fisik dengan catatan transaksi dan mencetak laporan ringkasan shift kasir.

---

## 📄 Lisensi

Hak Cipta &copy; 2026 Latansa Software. Didistribusikan di bawah lisensi terbuka [MIT](LICENSE).
