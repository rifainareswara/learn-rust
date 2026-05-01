// Modul 05: Modern Rust — Latihan
// Cara menjalankan: cargo run -p 05_modern_rust --bin latihan

fn main() {
    println!("=== Latihan 05: Modern Rust ===\n");
    latihan_1();
    latihan_2();
}

// ── Latihan 1 ──────────────────────────────────────────
// Buat custom error type `KalkulatorError` dengan varian:
//   - BagiNol
//   - AkarNegatif
// Implementasikan fmt::Display untuk KalkulatorError.
// Buat dua fungsi:
//   fn bagi(a: f64, b: f64) -> Result<f64, KalkulatorError>
//   fn akar(n: f64) -> Result<f64, KalkulatorError>  (gunakan n.sqrt())
// Buat fungsi ketiga yang menggunakan operator ? untuk memanggil keduanya:
//   fn hitung(a: f64, b: f64) -> Result<f64, KalkulatorError>
//   yang membagi a dengan b, lalu mengambil akar dari hasilnya.
fn latihan_1() {
    println!("--- Latihan 1: Custom Error & Operator ? ---");

    // TODO: definisikan KalkulatorError, bagi(), akar(), hitung()

    // TODO: panggil hitung() dengan beberapa kasus:
    // (16.0, 4.0) → √(16/4) = √4 = 2.0
    // (9.0, 0.0)  → Error: BagiNol
    // (-4.0, 1.0) → Error: AkarNegatif

    println!();
}

// ── Latihan 2 ──────────────────────────────────────────
// Buat program counter multi-thread:
// - Gunakan Arc<Mutex<u32>> sebagai shared counter
// - Spawn 10 thread, masing-masing menambah counter sebanyak 100
// - Tunggu semua thread selesai
// - Cetak nilai akhir counter (harus 1000)
fn latihan_2() {
    println!("--- Latihan 2: Arc + Mutex + Threads ---");

    // TODO: buat Arc<Mutex<u32>> counter
    // TODO: spawn 10 thread, masing-masing tambah 100x
    // TODO: join semua thread
    // TODO: cetak nilai akhir (harus 1000)

    println!();
}
