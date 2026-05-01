# 🦀 Belajar Rust — Kurikulum Terstruktur

Repositori ini berisi materi belajar Rust yang disusun secara bertahap dari dasar hingga membuat aplikasi CRUD nyata.

---

## 📚 Daftar Modul

| Modul | Topik | Perintah |
|-------|-------|----------|
| [01_dasar_rust](./01_dasar_rust/README.md) | Variables, Tipe Data, Fungsi, Control Flow | `cargo run -p modul_01_dasar_rust` |
| [02_ownership](./02_ownership/README.md) | Ownership, Borrowing, Slices | `cargo run -p modul_02_ownership` |
| [03_structs_enums](./03_structs_enums/README.md) | Structs, Enums, Pattern Matching | `cargo run -p modul_03_structs_enums` |
| [04_intermediate](./04_intermediate/README.md) | Traits, Generics, Collections, Lifetimes | `cargo run -p modul_04_intermediate` |
| [05_modern_rust](./05_modern_rust/README.md) | Error Handling, Smart Pointers, Concurrency | `cargo run -p modul_05_modern_rust` |
| [06_crud_app](./06_crud_app/README.md) | Axum, SQLite, REST API (Proyek Akhir) | `cargo run -p modul_06_crud_app` |

---

## 🚀 Cara Menggunakan

### Menjalankan Contoh (Materi)
```bash
cargo run -p modul_<nomor>_<nama>
# Contoh:
cargo run -p modul_01_dasar_rust
```

### Menjalankan Latihan
```bash
cargo run -p modul_<nomor>_<nama> --bin latihan
# Contoh:
cargo run -p modul_01_dasar_rust --bin latihan
```

### Melihat Solusi
```bash
cargo run -p modul_<nomor>_<nama> --bin solusi
# Contoh:
cargo run -p modul_01_dasar_rust --bin solusi
```

---

## 📋 Prasyarat

- [Rust & Cargo](https://rustup.rs/) terinstall
- Untuk modul 06: tidak perlu setup database tambahan (SQLite otomatis dibuat)

---

## 🗺️ Jalur Pembelajaran

```
01_dasar_rust → 02_ownership → 03_structs_enums → 04_intermediate → 05_modern_rust → 06_crud_app
```

Setiap modul memiliki:
- **README.md** — Penjelasan konsep dengan tabel dan contoh kode
- **src/main.rs** — Kode contoh yang bisa langsung dijalankan
- **src/bin/latihan.rs** — Soal latihan dengan template TODO
- **src/bin/solusi.rs** — Solusi lengkap dari latihan

---

## 🗂️ Struktur Folder

```
learn-rust/
├── Cargo.toml              ← Workspace
├── README.md               ← Panduan ini
├── 01_dasar_rust/
│   ├── Cargo.toml
│   ├── README.md           ← Materi konsep
│   └── src/
│       ├── main.rs         ← Contoh lengkap
│       └── bin/
│           ├── latihan.rs  ← Soal (TODO)
│           └── solusi.rs   ← Jawaban
├── 02_ownership/  ...
└── 06_crud_app/   ...
```
