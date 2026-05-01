# Modul 03 — Structs & Enums

## 🎯 Tujuan Pembelajaran

Setelah menyelesaikan modul ini, kamu akan memahami:
- Mendefinisikan dan menggunakan Struct
- Methods dan Associated Functions
- Enums dan variannya
- Pattern Matching dengan `match`
- `Option<T>` untuk nilai yang mungkin tidak ada

---

## 📖 Konsep Utama

### 1. Struct

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

// Membuat instance
let user = User {
    username: String::from("rifai"),
    email: String::from("rifai@example.com"),
    active: true,
};
```

**Tuple Struct** — struct tanpa nama field:
```rust
struct Color(i32, i32, i32);
let hitam = Color(0, 0, 0);
println!("R={}", hitam.0);
```

---

### 2. Methods (`impl`)

```rust
impl User {
    // Associated Function (seperti constructor)
    fn baru(username: String, email: String) -> User {
        User { username, email, active: true }
    }

    // Method (menerima &self)
    fn perkenalan(&self) {
        println!("Halo, saya {}!", self.username);
    }
}

let user = User::baru(String::from("rifai"), String::from("r@r.com"));
user.perkenalan();
```

---

### 3. Enums

Enum di Rust bisa menyimpan data di setiap varian:

```rust
enum Pesan {
    Keluar,
    Pindah { x: i32, y: i32 },
    Tulis(String),
    UbahWarna(i32, i32, i32),
}
```

---

### 4. Pattern Matching

```rust
let pesan = Pesan::Tulis(String::from("Halo!"));

match pesan {
    Pesan::Keluar => println!("Keluar"),
    Pesan::Pindah { x, y } => println!("Pindah ke ({}, {})", x, y),
    Pesan::Tulis(teks) => println!("Teks: {}", teks),
    Pesan::UbahWarna(r, g, b) => println!("Warna: {},{},{}", r, g, b),
}
```

---

### 5. Option\<T\>

Rust tidak punya `null`. Sebagai gantinya ada `Option<T>`:

```rust
enum Option<T> {
    None,
    Some(T),
}

let ada: Option<i32> = Some(5);
let tidak_ada: Option<i32> = None;

// Gunakan if let untuk case yang lebih simpel
if let Some(nilai) = ada {
    println!("Nilainya: {}", nilai);
}
```

---

## ▶️ Cara Menjalankan

```bash
cargo run -p 03_structs_enums
cargo run -p 03_structs_enums --bin latihan
cargo run -p 03_structs_enums --bin solusi
```

---

## ✅ Selanjutnya

Modul berikutnya: **[04_intermediate](../04_intermediate/README.md)** — Traits, Generics, dan Collections!
