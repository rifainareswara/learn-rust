# Modul 02 — Ownership & Borrowing

## 🎯 Tujuan Pembelajaran

Setelah menyelesaikan modul ini, kamu akan memahami:
- Konsep Ownership (kepemilikan) di Rust
- Move semantics vs Clone
- Borrowing dengan References (`&`)
- Mutable References (`&mut`)
- String Slices (`&str`)

---

## 📖 Konsep Utama

### 1. Aturan Ownership

Rust tidak punya Garbage Collector. Sebagai gantinya, ada **3 aturan ownership**:
1. Setiap nilai punya tepat satu **owner**
2. Hanya boleh ada satu owner di satu waktu
3. Ketika owner keluar dari scope, nilai akan di-drop (memori dibebaskan)

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 di-MOVE ke s2, s1 tidak bisa dipakai lagi!
// println!("{}", s1); // ERROR! s1 sudah tidak valid
```

---

### 2. Clone (Menduplikat data)

Untuk menyalin data yang ada di heap, gunakan `.clone()`:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // duplikat, kedua masih valid
println!("{} dan {}", s1, s2); // OK!
```

> **Catatan**: Tipe sederhana seperti `i32`, `bool`, `char` otomatis ter-Copy (tidak perlu `.clone()`).

---

### 3. References & Borrowing

Daripada memindahkan ownership, kita bisa **meminjam** dengan referensi:

```rust
let s = String::from("hello");

// Borrowing (tidak memindahkan ownership)
let len = hitung_panjang(&s);
println!("{} panjangnya {}", s, len); // s masih valid!

fn hitung_panjang(s: &String) -> usize {
    s.len()
} // s dipinjam, tidak di-drop di sini
```

---

### 4. Mutable References

```rust
let mut s = String::from("halo");
ubah_string(&mut s);

fn ubah_string(s: &mut String) {
    s.push_str(", dunia!");
}
```

> **Aturan penting**: Hanya boleh ada **SATU mutable reference** pada satu waktu!

---

### 5. String Slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // slice "hello"
let world = &s[6..11]; // slice "world"
```

---

## ▶️ Cara Menjalankan

```bash
cargo run -p 02_ownership
cargo run -p 02_ownership --bin latihan
cargo run -p 02_ownership --bin solusi
```

---

## ✅ Selanjutnya

Modul berikutnya: **[03_structs_enums](../03_structs_enums/README.md)** — Mendefinisikan tipe data sendiri!
