# Modul 05 — Modern Rust

## 🎯 Tujuan Pembelajaran

Setelah menyelesaikan modul ini, kamu akan memahami:
- Error Handling dengan `Result<T, E>` dan operator `?`
- Smart Pointers: `Box<T>`, `Rc<T>`, `Arc<T>`
- Concurrency dasar dengan threads
- Perkenalan Async/Await

---

## 📖 Konsep Utama

### 1. Result\<T, E\> — Error Handling

Di Rust, tidak ada `try-catch`. Gunakan `Result`:

```rust
fn bagi(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Tidak bisa dibagi nol!"))
    } else {
        Ok(a / b)
    }
}

match bagi(10.0, 2.0) {
    Ok(hasil) => println!("Hasil: {}", hasil),
    Err(e) => println!("Error: {}", e),
}
```

---

### 2. Operator `?` (Propagasi Error)

`?` mempercepat penulisan error handling — jika `Err`, langsung return:

```rust
use std::fs;
use std::io;

fn baca_file(path: &str) -> Result<String, io::Error> {
    let isi = fs::read_to_string(path)?; // jika Err, langsung return Err
    Ok(isi)
}
```

---

### 3. Smart Pointers

**`Box<T>`** — simpan data di Heap:
```rust
let b = Box::new(5); // angka 5 di heap
println!("b = {}", b);
```

**`Rc<T>`** — banyak owner, satu thread:
```rust
use std::rc::Rc;
let data = Rc::new(String::from("bersama"));
let owner1 = Rc::clone(&data);
let owner2 = Rc::clone(&data);
println!("Jumlah owner: {}", Rc::strong_count(&data)); // 3
```

**`Arc<T>`** — banyak owner, multi-thread (Atomic):
```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(String::from("thread-safe"));
let clone_for_thread = Arc::clone(&data);

thread::spawn(move || {
    println!("Thread: {}", clone_for_thread);
}).join().unwrap();
```

---

### 4. Concurrency

```rust
use std::thread;
use std::sync::{Mutex, Arc};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..5 {
    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = c.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for h in handles { h.join().unwrap(); }
println!("Counter: {}", *counter.lock().unwrap()); // 5
```

---

### 5. Async/Await

```rust
// Membutuhkan runtime seperti Tokio
#[tokio::main]
async fn main() {
    let hasil = ambil_data().await;
    println!("{}", hasil);
}

async fn ambil_data() -> String {
    // Simulasi operasi async
    String::from("Data berhasil diambil!")
}
```

---

## ▶️ Cara Menjalankan

```bash
cargo run -p 05_modern_rust
cargo run -p 05_modern_rust --bin latihan
cargo run -p 05_modern_rust --bin solusi
```

---

## ✅ Selanjutnya

Modul berikutnya: **[06_crud_app](../06_crud_app/README.md)** — Proyek akhir: REST API dengan Axum & SQLite!
