# 06. Packaging & Distribusi Lintas Platform (NSIS, Linux, macOS)

Dokumen ini memuat konfigurasi build dan strategi pengemasan installer aplikasi POS Kasir Desktop menggunakan toolchain bawaan **Tauri v2**.

---

## 1. Target Format Distribusi

| Sistem Operasi | Target Format | Deskripsi |
| :--- | :--- | :--- |
| **Windows** | **NSIS (`.exe`)** | Installer Nullsoft resmi dengan wizard instalasi, shortcut desktop, dan pilihan direktori kustom. |
| **Linux (Debian / Ubuntu / Mint)** | **`.deb`** | Paket biner siap instal via `dpkg -i` atau APT software center. |
| **Linux (Fedora / RHEL / openSUSE)** | **`.rpm`** | Paket biner siap instal via `rpm -i` atau DNF/Zypper. |
| **Linux (Arch Linux / Manjaro)** | **`pacman` (`.pkg.tar.zst`) / AUR** | Paket native pacman dan skrip `PKGBUILD` untuk rilis di Arch User Repository. |
| **Linux Universal** | **`AppImage`** | Single-file executable portabel tanpa perlu proses instalasi dependensi. |
| **macOS** | **`.dmg`** | Image drag-and-drop ke folder `/Applications` (arsitektur Universal Intel x64 & Apple Silicon arm64). |

---

## 2. Konfigurasi `tauri.conf.json`

Berikut cuplikan konfigurasi bundle installer pada file `src-tauri/tauri.conf.json`:

```json
{
  "productName": "POS Kasir Desktop",
  "version": "1.0.0",
  "identifier": "com.poskasir.desktop",
  "build": {
    "frontendDist": "../build"
  },
  "bundle": {
    "active": true,
    "targets": ["nsis", "deb", "rpm", "appimage", "dmg"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "windows": {
      "nsis": {
        "installMode": "both",
        "languages": ["English", "Indonesian"],
        "displayLanguageSelector": false,
        "installerIcon": "icons/icon.ico",
        "headerImage": "icons/nsis-header.bmp",
        "sidebarImage": "icons/nsis-sidebar.bmp"
      },
      "webviewInstallMode": {
        "type": "downloadBootstrapper"
      }
    },
    "linux": {
      "deb": {
        "depends": ["libwebkit2gtk-4.1-0"]
      },
      "rpm": {
        "depends": ["webkit2gtk4.1"]
      },
      "appimage": {
        "bundleMediaFramework": false
      }
    },
    "macOS": {
      "frameworks": [],
      "minimumSystemVersion": "10.15"
    }
  }
}
```

---

## 3. Fitur Khusus Installer Windows (NSIS)

1. **Fleksibilitas Direktori Instalasi**:
   - Pengguna dapat memilih lokasi instalasi kustom (misal di `C:\POSKasir` atau `D:\Program\POSKasir`) atau lokasi default sistem.
2. **Shortcut Otomatis**:
   - Otomatis membuat shortcut di Desktop dan Start Menu Windows.
3. **Penyelamatan Basis Data Saat Uninstall/Update**:
   - Skrip uninstaller NSIS dikonfigurasi untuk **tidak menghapus** file basis data SQLite (`pos_kasir.db`) di folder AppData pengguna, sehingga data riwayat penjualan dan stok tetap aman saat aplikasi di-update atau diinstal ulang.
4. **Auto-Detect WebView2 Runtime**:
   - Jika aplikasi dijalankan di Windows 7/8 yang belum memiliki Microsoft Edge WebView2, installer NSIS otomatis mengunduh dan memasang *Evergreen Bootstrapper* WebView2 resmi dari Microsoft.

---

## 4. Perintah Build (CLI Commands)

```bash
# Build untuk OS yang sedang digunakan:
npm run tauri build

# Hasil build pada Windows:
# Output: src-tauri/target/release/bundle/nsis/POS Kasir Desktop_1.0.0_x64-setup.exe

# Hasil build pada Linux:
# Output .deb:      src-tauri/target/release/bundle/deb/pos-kasir-desktop_1.0.0_amd64.deb
# Output .rpm:      src-tauri/target/release/bundle/rpm/pos-kasir-desktop-1.0.0-1.x86_64.rpm
# Output AppImage:  src-tauri/target/release/bundle/appimage/pos-kasir-desktop_1.0.0_amd64.AppImage
```
