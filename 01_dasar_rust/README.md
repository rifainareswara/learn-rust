# Modul 01 — Dasar Rust

## 🎯 Tujuan Pembelajaran

Setelah menyelesaikan modul ini, kamu akan memahami:
- Variables dan Mutability
- Tipe Data dasar (scalar & compound)
- Fungsi (parameters, return values)
- Control Flow (if, loop, while, for)

---

## 📖 Konsep Utama

### 1. Variables & Mutability

Di Rust, semua variabel secara default **immutable** (tidak bisa diubah).
Gunakan `mut` untuk membuat variabel yang bisa diubah.

```rust
let x = 5;          // immutable
let mut y = 10;     // mutable
const MAX: u32 = 100; // constant (huruf besar, tipe wajib)
```

> **Shadowing** — kamu bisa mendeklarasi ulang variabel dengan nama yang sama menggunakan `let`:
> ```rust
> let x = 5;
> let x = x + 1; // x sekarang 6
> ```

---

### 2. Tipe Data

**Scalar** (satu nilai):
| Tipe | Contoh | Keterangan |
|------|--------|------------|
| `i32` / `u32` | `let n: i32 = -5;` | Integer (signed/unsigned) |
| `f64` | `let f = 3.14;` | Float (default) |
| `bool` | `let b = true;` | Boolean |
| `char` | `let c = 'A';` | Karakter Unicode |

**Compound** (kumpulan nilai):
```rust
let tuple: (i32, f64, bool) = (500, 6.4, true);
let array = [1, 2, 3, 4, 5]; // ukuran tetap
```

---

### 3. Fungsi

```rust
fn nama_fungsi(param: TipeData) -> TipeKembalian {
    // body
    ekspresi_terakhir // tanpa titik koma = return value
}
```

---

### 4. Control Flow

```rust
// if-else
if kondisi { ... } else if ... { ... } else { ... }

// loop (infinite, bisa break dengan nilai)
let result = loop {
    break 42;
};

// while
while kondisi { ... }

// for (paling idiomatik di Rust)
for item in koleksi { ... }
for n in 0..5 { ... }  // 0,1,2,3,4
```

---

## ▶️ Cara Menjalankan

```bash
# Contoh materi
cargo run -p 01_dasar_rust

# Latihan
cargo run -p 01_dasar_rust --bin latihan

# Solusi
cargo run -p 01_dasar_rust --bin solusi
```

---

## 📝 Latihan

Lihat file [`src/bin/latihan.rs`](./src/bin/latihan.rs) untuk soal-soal latihan.

## ✅ Selanjutnya

Modul berikutnya: **[02_ownership](../02_ownership/README.md)** — Konsep paling unik di Rust!
