// Modul 04: Intermediate Rust — Latihan
// Cara menjalankan: cargo run -p 04_intermediate --bin latihan

use std::collections::HashMap;

fn main() {
    println!("=== Latihan 04: Intermediate Rust ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

// ── Latihan 1 ──────────────────────────────────────────
// Buat trait `Hitung` dengan method:
//   fn luas(&self) -> f64;
//   fn keliling(&self) -> f64;
// Implementasikan untuk struct `Lingkaran { jari_jari: f64 }`
// dan struct `Persegi { sisi: f64 }`.
// Buat fungsi generic yang mencetak luas dan keliling.
fn latihan_1() {
    println!("--- Latihan 1: Traits & Generics ---");

    // TODO: definisikan trait Hitung, struct Lingkaran & Persegi, lalu implementasikan

    // TODO: panggil fungsi generic untuk masing-masing bentuk

    println!();
}

// ── Latihan 2 ──────────────────────────────────────────
// Buat Vec<String> berisi 5 nama buah.
// Gunakan iterator untuk:
//   (a) cetak semua buah dengan huruf kapital (.to_uppercase())
//   (b) filter dan kumpulkan yang panjangnya > 5 karakter
//   (c) cetak jumlah total karakter semua nama buah
fn latihan_2() {
    println!("--- Latihan 2: Vec & Iterators ---");
    let buah = vec!["apel", "mangga", "durian", "jeruk", "semangka"];

    // TODO (a): cetak semua dengan huruf kapital
    // for b in &buah { println!("  {}", b.to_uppercase()); }

    // TODO (b): filter panjang > 5, collect ke Vec<&&str>
    // let panjang: Vec<&&str> = ...;
    // println!("  Nama panjang (>5 huruf): {:?}", panjang);

    // TODO (c): hitung total karakter
    // let total: usize = ...;
    // println!("  Total karakter: {}", total);

    println!();
}

// ── Latihan 3 ──────────────────────────────────────────
// Buat HashMap yang memetakan nama mahasiswa ke nilai ujian (Vec<u32>).
// Setelah mengisi datanya:
//   (a) hitung dan cetak rata-rata nilai tiap mahasiswa
//   (b) temukan mahasiswa dengan rata-rata tertinggi
fn latihan_3() {
    println!("--- Latihan 3: HashMap ---");
    let mut nilai: HashMap<String, Vec<u32>> = HashMap::new();

    // TODO: isi data
    // nilai.insert(String::from("Rifai"), vec![80, 90, 85]);
    // nilai.insert(String::from("Budi"), vec![70, 75, 80]);
    // nilai.insert(String::from("Siti"), vec![95, 92, 98]);

    // TODO (a): cetak rata-rata tiap mahasiswa

    // TODO (b): temukan mahasiswa dengan rata-rata tertinggi
    // dan cetak: "Tertinggi: [nama] dengan rata-rata [nilai]"

    println!();
}
