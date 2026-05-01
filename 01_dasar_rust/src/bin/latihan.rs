// Modul 01: Dasar Rust — Latihan
// Cara menjalankan: cargo run -p 01_dasar_rust --bin latihan
//
// INSTRUKSI: Lengkapi setiap TODO di bawah ini.
// Jangan lihat solusi sebelum mencoba sendiri!

fn main() {
    println!("=== Latihan 01: Dasar Rust ===\n");

    latihan_1();
    latihan_2();
    latihan_3();
    latihan_4();
}

// ── Latihan 1 ──────────────────────────────────────────
// Buat sebuah variabel immutable `nama` berisi namamu,
// variabel mutable `umur` berisi umurmu,
// lalu tambahkan 1 ke umur dan cetak kedua variabel.
fn latihan_1() {
    println!("--- Latihan 1: Variables ---");

    // TODO: deklarasikan variabel `nama` (immutable) berisi namamu
    // let nama = ...;

    // TODO: deklarasikan variabel `umur` (mutable) berisi umurmu
    // let mut umur = ...;

    // TODO: tambah 1 ke umur
    // umur += 1;

    // TODO: cetak: "Nama: [nama], Umur: [umur]"
    // println!(...);

    println!(); // jangan hapus baris ini
}

// ── Latihan 2 ──────────────────────────────────────────
// Buat tuple yang menyimpan (nama_kota: &str, populasi: u32, luas_km2: f64)
// untuk kota Jakarta. Lalu cetak setiap elemennya.
fn latihan_2() {
    println!("--- Latihan 2: Tuple ---");

    // TODO: buat tuple `jakarta` dengan data:
    //   nama = "Jakarta", populasi = 10_500_000, luas = 664.01
    // let jakarta = ...;

    // TODO: cetak "Kota: [nama], Populasi: [pop], Luas: [luas] km²"
    // println!(...);

    println!();
}

// ── Latihan 3 ──────────────────────────────────────────
// Buat fungsi `celcius_ke_fahrenheit(c: f64) -> f64`
// dengan rumus: F = (C × 9/5) + 32
// Lalu panggil dengan beberapa nilai dan cetak hasilnya.
fn latihan_3() {
    println!("--- Latihan 3: Fungsi ---");

    // TODO: panggil celcius_ke_fahrenheit dengan 0.0, 100.0, 37.0
    // dan cetak hasilnya dalam format: "[C]°C = [F]°F"

    println!();
}

// TODO: definisikan fungsi celcius_ke_fahrenheit di sini
// fn celcius_ke_fahrenheit(c: f64) -> f64 {
//     ...
// }

// ── Latihan 4 ──────────────────────────────────────────
// Gunakan loop for untuk mencetak bilangan 1-10,
// tapi untuk kelipatan 3 cetak "Fizz",
// untuk kelipatan 5 cetak "Buzz",
// untuk keduanya cetak "FizzBuzz"
fn latihan_4() {
    println!("--- Latihan 4: FizzBuzz ---");

    // TODO: for n in 1..=10 { ... }

    println!();
}
