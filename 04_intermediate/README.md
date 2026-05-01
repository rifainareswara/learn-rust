# Modul 04 — Intermediate Rust

## 🎯 Tujuan Pembelajaran

Setelah menyelesaikan modul ini, kamu akan memahami:
- Traits (seperti interface)
- Generics (kode yang bekerja untuk banyak tipe)
- Collections: `Vec<T>` dan `HashMap<K, V>`
- Lifetimes dasar

---

## 📖 Konsep Utama

### 1. Traits

Trait mendefinisikan perilaku yang bisa dibagi antar tipe:

```rust
trait Ringkasan {
    fn ringkas(&self) -> String;
    
    // Default implementation
    fn panjang_ringkasan(&self) -> usize {
        self.ringkas().len()
    }
}

struct Artikel { judul: String, penulis: String }

impl Ringkasan for Artikel {
    fn ringkas(&self) -> String {
        format!("{} oleh {}", self.judul, self.penulis)
    }
}
```

---

### 2. Generics

Kode yang bisa bekerja dengan berbagai tipe data:

```rust
// Fungsi generic dengan trait bound
fn tampilkan<T: Ringkasan>(item: &T) {
    println!("Berita: {}", item.ringkas());
}

// Alternatif dengan 'impl Trait'
fn tampilkan(item: &impl Ringkasan) {
    println!("Berita: {}", item.ringkas());
}
```

---

### 3. Vec\<T\> — Vektor

```rust
let mut angka: Vec<i32> = Vec::new();
angka.push(1);
angka.push(2);
angka.push(3);

// Atau gunakan macro
let angka = vec![1, 2, 3, 4, 5];

// Iterasi
for n in &angka {
    println!("{}", n);
}

// Akses aman dengan .get()
match angka.get(100) {
    Some(v) => println!("{}", v),
    None => println!("Index tidak ada"),
}
```

---

### 4. HashMap\<K, V\>

```rust
use std::collections::HashMap;

let mut skor: HashMap<String, i32> = HashMap::new();
skor.insert(String::from("Alice"), 100);
skor.insert(String::from("Bob"), 85);

// Akses nilai
let nilai = skor.get("Alice").copied().unwrap_or(0);

// Entry API — insert hanya jika belum ada
skor.entry(String::from("Charlie")).or_insert(70);
```

---

### 5. Lifetimes (Dasar)

Lifetime memastikan referensi selalu valid:

```rust
// 'a adalah anotasi lifetime
// Artinya: hasil kembalian valid selama x dan y valid
fn terpanjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## ▶️ Cara Menjalankan

```bash
cargo run -p 04_intermediate
cargo run -p 04_intermediate --bin latihan
cargo run -p 04_intermediate --bin solusi
```

---

## ✅ Selanjutnya

Modul berikutnya: **[05_modern_rust](../05_modern_rust/README.md)** — Error Handling, Smart Pointers, dan Async!
