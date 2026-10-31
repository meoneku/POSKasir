# 03. Alur Bisnis Menyeluruh: Dari Hulu ke Hilir

Dokumen ini membedah siklus operasional bisnis ritel dan grosir dari awal pengadaan barang (hulu), pengelolaan stok di gudang (tengah), hingga transaksi penjualan dan penutupan buku kasir (hilir).

---

## 1. Diagram Siklus Hulu ke Hilir

```mermaid
flowchart TD
    subgraph Hulu["1. Hulu: Pengadaan (Procurement)"]
        SUP[Supplier / Pemasok] --> PO[Purchase Order / PO]
        PO --> GR[Penerimaan Barang / Good Receipt]
        GR --> AP[Hutang Dagang / AP]
        GR --> STK_IN[Barang Masuk Gudang]
    end

    subgraph Tengah["2. Tengah: Inventaris & Multi-Tier"]
        STK_IN --> WAREHOUSE[(Penyimpanan & Kartu Stok)]
        WAREHOUSE --> CONVERT[Konversi Multi-Satuan:\n1 Dus = 12 Pak = 48 Pcs]
        WAREHOUSE --> PRICING[Matrix Tier Harga:\nEceran, Grosir 1, Grosir 2, Member]
        WAREHOUSE --> OPNAME[Stok Opname Digital]
    end

    subgraph HilirA["3. Hilir A: Kasir Retail (Penjualan Eceran)"]
        PRICING --> POS_RETAIL[Kasir Cepat POS]
        POS_RETAIL --> SCAN[Scan Barcode / Shortcut F2]
        POS_RETAIL --> PAY_RETAIL[Bayar: Tunai / QRIS / EDC]
        PAY_RETAIL --> RECEIPT[Cetak Struk Thermal 58/80mm\n& Buka Laci Uang via RJ11]
    end

    subgraph HilirB["4. Hilir B: Penjualan Partai Besar (Grosir / B2B)"]
        PRICING --> POS_WHOLESALE[Order Penjualan Partai Besar]
        POS_WHOLESALE --> CUST_B2B[Data Toko Mitra / Pelanggan]
        POS_WHOLESALE --> CREDIT[Penjualan Tempo / Piutang - AR]
        POS_WHOLESALE --> DO_INV[Cetak Surat Jalan & Faktur A4]
    end

    subgraph Penutupan["5. Kasir & Keuangan Operasional"]
        RECEIPT --> SHIFT_CLOSE[Tutup Shift (Blind Close)]
        PAY_RETAIL --> CASH_RECON[Rekonsiliasi Kas & Laba Kotor]
        CREDIT --> AR_SETTLE[Pelunasan Piutang Pelanggan]
        AP --> AP_SETTLE[Pembayaran Hutang Supplier]
    end
```

---

## 2. Rincian Modul Bisnis

### A. Modul Hulu: Manajemen Pemasok & Pengadaan
1. **Master Supplier**:
   - Menyimpan data kontak, alamat, nomor telepon/WhatsApp, NPWP supplier, serta ketentuan tempo pembayaran (COD, 7 hari, 14 hari, 30 hari).
2. **Purchase Order (PO)**:
   - Dibuat saat stok produk berada di bawah batas minimum (*min_stock_alert*).
   - Status PO: `draft` $\rightarrow$ `ordered` $\rightarrow$ `received` $\rightarrow$ `cancelled`.
3. **Penerimaan Barang (Good Receipt)**:
   - Pencocokan barang fisik yang datang dengan dokumen PO.
   - Perhitungan HPP otomatis menggunakan metode rata-rata tertimbang (*Weighted Moving Average*).
   - Menambah stok secara instan pada basis data.
4. **Hutang Usaha (Accounts Payable)**:
   - Pelacakan faktur supplier yang belum lunas.
   - Peringatan jatuh tempo hutang untuk menjaga hubungan baik dengan pemasok.
5. **Retur Pembelian**:
   - Pencatatan barang rusak/kadaluarsa yang dikembalikan ke supplier untuk memotong sisa hutang.

---

### B. Modul Tengah: Manajemen Persediaan & Multi-Satuan/Harga
1. **Hierarki Konversi Satuan**:
   - Memungkinkan pembelian dalam satuan besar (Dus) tetapi dapat dijual dalam satuan sedang (Pak) atau satuan kecil (Pcs).
   - **Contoh Konversi**:
     - Satuan Dasar: `Pcs` (Faktor: 1)
     - Satuan Menengah: `Pak` (Faktor: 12 Pcs)
     - Satuan Besar: `Dus` (Faktor: 48 Pcs)
   - Setiap mutasi pengurangan stok selalu dikonversi ke unit dasar sehingga tidak pernah terjadi selisih fraksional.
2. **Matrix Multi-Tier Harga**:
   - **Harga Eceran**: Harga dasar per unit untuk pembeli biasa.
   - **Harga Grosir 1**: Harga khusus untuk pembelian volume sedang (misal pembelian $\ge 12$ pcs).
   - **Harga Grosir 2**: Harga khusus untuk pembelian partai besar (misal pembelian $\ge 1$ Dus).
   - **Harga Member / Toko Langganan**: Harga negosiasi khusus yang terikat pada pelanggan tertentu.
3. **Kartu Stok & Jejak Audit**:
   - Setiap mutasi masuk dan keluar mencatat riwayat lengkap: `Waktu, No Referensi, Jenis Mutasi, Qty Masuk, Qty Keluar, Sisa Stok, Operator`.
4. **Stok Opname Digital**:
   - Form verifikasi fisik barang berkala dengan pencatatan selisih lebih/kurang (*variance*) dan persetujuan penyesuaian oleh Supervisor.

---

### C. Modul Hilir: Kasir Retail (Penjualan Eceran Cepat)
1. **Ergonomi Kasir Berkecepatan Tinggi**:
   - Navigasi penuh via keyboard:
     - `F2`: Pencarian Cepat Produk / Pindah ke Kolom Barcode
     - `F4`: Ubah Kuantitas Baris Aktif
     - `F8`: Tahan Pesanan Sementara (*Hold*)
     - `F9`: Buka Kembali Pesanan yang Ditahan (*Recall*)
     - `F12`: Buka Dialog Pembayaran
     - `ESC`: Batal / Tutup Jendela Dialog
2. **Scanner Barcode USB HID**:
   - Listener global menangkap input scan tanpa mengharuskan kursor aktif di input pencarian.
3. **Opsi Pembayaran Lengkap**:
   - Tunai (dengan tombol nominal uang pas dan pecahan cepat Rp 10.000 s/d Rp 100.000).
   - Non-Tunai: QRIS (tampil di layar / cetak di struk), EDC, Transfer Bank.
   - Pembayaran Campuran (*Split Payment*: sebagian tunai + sebagian non-tunai).
4. **Hardware Kasir**:
   - Cetak struk instan ke thermal printer 58mm/80mm.
   - Otomatis membuka laci uang kasir (*cash drawer*) melalui pulsa sinyal RJ11.

---

### D. Modul Hilir: Penjualan Partai Besar & Grosir (B2B)
1. **Database Mitra & Plafon Kredit (Credit Limit)**:
   - Data toko pelanggan, alamat pengiriman, NPWP/NIK, dan batas plafon piutang.
   - Peringatan dan blokir transaksi otomatis jika saldo piutang melebihi plafon tanpa verifikasi PIN Supervisor.
2. **Surat Jalan (Delivery Order / DO)**:
   - Dokumen serah terima fisik barang untuk armada pengiriman tanpa mencantumkan nilai rupiah.
3. **Faktur Penjualan (Invoice A4 / Continuous Form)**:
   - Format cetak formal A4 atau Continuous Form kertas rangkap (NCR) untuk arsip pembukuan resmi.
4. **Pelunasan Piutang Bertahap**:
   - Pelanggan dapat membayar piutang secara bertahap atau lunas dengan bukti kwitansi tanda terima resmi.

---

### E. Siklus Shift Kasir & Rekonsiliasi Kas
1. **Buka Shift**:
   - Kasir menginput modal awal kas di laci kasir (*Starting Cash Drawer*).
2. **Pencatatan Kas Masuk / Keluar Operasional**:
   - Kas masuk: Tambahan uang receh kembalian dari brankas toko.
   - Kas keluar: Pengeluaran operasional kecil kasir (misal: pembayaran iuran sampah, pembelian perlengkapan kasir).
3. **Tutup Shift (Blind Close)**:
   - Kasir menghitung dan memasukkan uang fisik yang ada di laci tanpa melihat angka ringkasan sistem terlebih dahulu (mencegah kecurangan).
   - Sistem membandingkan uang fisik dengan total transaksi dan menampilkan selisih (*Lebih / Pas / Kurang*).
   - Mencetak laporan X/Z report pada printer thermal.
