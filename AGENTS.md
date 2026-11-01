# AGENTS.md — Pedoman Utama & Konteks AI Agent

Dokumen ini merupakan panduan utama (*master context & guidelines*) bagi setiap AI Agent yang bekerja, mengembangkan, memodifikasi, atau memelihara repositori **POS Kasir Desktop**.

---

## 1. Ikhtisar Repositori & Spesifikasi Inti

- **Nama Proyek**: POS Kasir Desktop Multi-Platform
- **Desktop Runtime**: **Tauri v2 (Rust)**
- **Frontend**: **Svelte 5 (Runes)** + **Tailwind CSS** + **lucide-svelte**
- **Basis Data**: **SQLite (Mode WAL / Write-Ahead Logging)**
- **Model Jaringan**: **PC Kasir Utama sebagai Server Lokal (Embedded Axum REST & WebSocket Server, Port 8080)** dengan mDNS Auto-Discovery. *Zero external DB dependencies* (pengguna tidak perlu memasang PostgreSQL/MySQL).
- **Target Packaging**:
  - Windows: **NSIS Installer (`.exe`)**
  - Linux: **`.deb`** (Debian/Ubuntu), **`.rpm`** (Fedora/RHEL), **`pacman` / PKGBUILD** (Arch Linux), **`AppImage`** (Universal)
  - macOS: **`.dmg`** (Universal binary x64/arm64)

---

## 2. Peta Dokumentasi Lengkap (`docs/`)

Setiap detail arsitektur, skema tabel, dan alur bisnis telah dipecah secara mendalam ke dalam dokumen terpisah. AI Agent **wajib merujuk ke dokumen-dokumen berikut**:

| File Dokumentasi | Ringkasan Isi |
| :--- | :--- |
| [docs/01-architecture-and-stack.md](file:///d:/Project/PosKasir/docs/01-architecture-and-stack.md) | Arsitektur teknis Tauri v2, performa memori (<50MB RAM), SQLite WAL, embedded Axum server, dan sinkronisasi LAN. |
| [docs/02-database-schema.md](file:///d:/Project/PosKasir/docs/02-database-schema.md) | Skema DDL lengkap seluruh tabel relasional (Pengaturan, Pajak, Pengguna, Supplier, PO, Produk, Multi-Satuan, Pelanggan, Shift, Transaksi). |
| [docs/03-business-flow-hulu-hilir.md](file:///d:/Project/PosKasir/docs/03-business-flow-hulu-hilir.md) | Siklus bisnis komprehensif: Pengadaan supplier, konversi multi-satuan/tier harga, kasir retail cepat, penjualan grosir B2B/piutang/faktur, dan tutup shift. |
| [docs/04-ui-ux-design-system.md](file:///d:/Project/PosKasir/docs/04-ui-ux-design-system.md) | Pedoman desain UI (tanpa ungu, tanpa gradasi, warna solid Emerald/Slate), perilaku UX desktop alami, shortcut, dan skeleton loaders. |
| [docs/05-customization-and-taxation.md](file:///d:/Project/PosKasir/docs/05-customization-and-taxation.md) | Mesin kustomisasi toko (profil, logo, formula nomor nota, metode bayar) dan mesin perpajakan (PPN 11%/12%, Inclusive/Exclusive, BKP vs Non-BKP). |
| [docs/06-packaging-and-distribution.md](file:///d:/Project/PosKasir/docs/06-packaging-and-distribution.md) | Konfigurasi pengemasan installer Windows NSIS, Linux (.deb, .rpm, pacman, AppImage), dan macOS (.dmg). |
| [docs/07-multi-branch-sync.md](file:///d:/Project/PosKasir/docs/07-multi-branch-sync.md) | Arsitektur sinkronisasi multi-cabang (Hub & Spoke / Mesh VPN), offline-first edge database, mutasi stok antar cabang, dan resolusi konflik. |

---

## 3. Aturan Mutlak & Batasan Desain (Non-Negotiable Rules)

Setiap AI Agent yang membuat atau memodifikasi kode **WAJIB MEMATUHI ATURAN INI**:

### A. Aturan UI & Styling
1. ❌ **DILARANG MENGGUNAKAN WARNA UNGU / VIOLET / MAGENTA** di bagian mana pun dalam aplikasi.
2. ❌ **DILARANG MENGGUNAKAN WARNA GRADASI (NO GRADIENTS)**: Dilarang menggunakan class `bg-gradient-*` atau CSS linear-gradient. Gunakan warna solid (*flat*) bersih dengan border 1px presisi dan micro-shadow.
3. ✔️ **Palet Warna Utama**: Gunakan warna **Emerald Green** (`#059669` / `#10b981`) sebagai warna aksen utama, dipadukan dengan **Slate/Zinc** netral (`#0f172a` teks gelap, `#f8fafc` kanvas terang, `#09090b` kanvas gelap).
4. ✔️ **Skeleton Loader Wajib**: Setiap perpindahan halaman atau proses pemuatan data wajib menampilkan efek **Skeleton Loader** (`animate-pulse`) yang halus (tanpa *layout shift* atau layar kosong sesaat). Durasi transisi rute maksimal 150 milidetik (*smooth fade-in*).

### B. Aturan UX & Ergonomi Kasir
1. **Shortcut Keyboard Konsisten**:
   - `F2`: Fokus pencarian produk / barcode.
   - `F4`: Edit kuantitas baris aktif.
   - `F8`: Tahan nota (*Hold*).
   - `F9`: Buka kembali nota tertahan (*Recall*).
   - `F12`: Buka pembayaran & selesaikan.
   - `ESC`: Batal / Tutup dialog.
2. **Autofokus Cerdas**: Saat dialog pembayaran terbuka, input nominal uang wajib langsung terfokus dan teks langsung ter-blok (*highlighted*).
3. **Global Barcode Listener**: Scanner barcode USB HID harus dapat langsung memasukkan item ke keranjang belanja dari layar kasir manapun tanpa harus mengklik kolom pencarian secara manual.

### C. Aturan Arsitektur & Kinerja
1. **Budget Memori**: Aplikasi harus tetap ringan dengan konsumsi RAM di bawah 50 MB saat idle. Hindari dependensi frontend yang membengkak (*bloatware*).
2. **Kemandirian Database**: Jangan pernah menambahkan dependensi yang mengharuskan pengguna menginstal server database eksternal (seperti service MySQL atau PostgreSQL di Windows Service). Seluruh data tersimpan di SQLite dengan mode WAL aktif.
3. **Atomic Transactions**: Seluruh transaksi penjualan, pengurangan stok, dan pencatatan mutasi kas wajib dieksekusi dalam satu transaksi atomik basis data (`BEGIN TRANSACTION` ... `COMMIT`).

---

## 4. Standar Kode & Konvensi Teknis

### A. Svelte 5 (Frontend)
- Gunakan fitur modern **Runes** Svelte 5:
  - `$state(...)` untuk state reaktif.
  - `$derived(...)` untuk nilai komputasi.
  - `$props()` untuk deklarasi komponen props.
- Komponen harus modular, ringkas, dan memisahkan logika UI dari panggilan IPC Tauri.

### B. Rust & Tauri v2 (Backend)
- Struktur command Tauri berada di dalam modul `src-tauri/src/commands/`.
- Layanan database SQLite dikelola dalam modul `src-tauri/src/db/` menggunakan *connection pooling* dan mode WAL.
- Server lokal Axum dikelola dalam modul `src-tauri/src/server/` dan hanya aktif jika opsi *Server Kasir Utama* diaktifkan di konfigurasi.
- Error handling wajib menggunakan tipe `Result<T, AppError>` dengan pesan kesalahan yang jelas dan bersahabat.

---

## 5. Status Roadmap Pengembangan

- [x] **Perencanaan Sistem Lengkap & Dokumentasi** (Tersedia di `docs/`)
- [x] **Sprint 1**: Inisialisasi Project (Tauri v2 + Svelte 5 + Tailwind + SQLite WAL Runner)
- [x] **Sprint 2**: Embedded Server Kasir Lokal (Axum + mDNS + WebSocket Sync)
- [ ] **Sprint 3**: Modul Hulu (Pemasok, PO, Penerimaan Barang, Multi-Satuan & Harga)
- [ ] **Sprint 4**: Modul Hilir Retail (Kasir Cepat, Barcode, Struk Thermal, Cash Drawer, Shift)
- [ ] **Sprint 5**: Modul Hilir Grosir (Pelanggan B2B, Piutang Tempo, Surat Jalan, Faktur Pajak)
- [ ] **Sprint 6**: Modul Multi-Cabang & Sinkronisasi (Mutasi Stok Antar Cabang, Cloud Relay / Mesh VPN Sync)
- [ ] **Sprint 7**: Build & Packaging Multi-Platform (NSIS Windows, Linux .deb/.rpm/pacman, macOS .dmg)
