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
    let nama = "rifai";

    // TODO: deklarasikan variabel `umur` (mutable) berisi umurmu
    // let mut umur = ...;
    let mut umur = 30;

    // TODO: tambah 1 ke umur
    // umur += 1;
    umur += 1;

    // TODO: cetak: "Nama: [nama], Umur: [umur]"
    // println!(...);
    println!("Nama: {}", nama);
    println!("Umur: {}", umur);

    println!(); // jangan hapus baris ini

    let nama_panjang = "Rizqi Nur Rifai";
    let mut umur_sekarang = 30;
    umur_sekarang += 1;

    println!("Nama Panjang : {}", nama_panjang);
    println!("Umur Sekarang : {}", umur_sekarang);
}

// ── Latihan 2 ──────────────────────────────────────────
// Buat tuple yang menyimpan (nama_kota: &str, populasi: u32, luas_km2: f64)
// untuk kota Jakarta. Lalu cetak setiap elemennya.
fn latihan_2() {
    println!("--- Latihan 2: Tuple ---");

    // TODO: buat tuple `jakarta` dengan data:
    //   nama = "Jakarta", populasi = 10_500_000, luas = 664.01
    // let jakarta = ...;
    let jakarta: (&str, u32, f64) = ("Jakarta", 10_500_000, 664.01);


    // TODO: cetak "Kota: [nama], Populasi: [pop], Luas: [luas] km²"
    // println!(...);
    println!("Kota: {}, Populasi {}, Luas {}", jakarta.0, jakarta.1, jakarta.2);
    println!();

    let bandung = ("Bandung", 20_200_200, 555.55, true);
    println!("Kota: {}, Populasi: {}, Luas: {}, Dipulau jawa: {}", bandung.0, bandung.1, bandung.2, bandung.3);
}

// ── Latihan 3 ──────────────────────────────────────────
// Buat fungsi `celcius_ke_fahrenheit(c: f64) -> f64`
// dengan rumus: F = (C × 9/5) + 32
// Lalu panggil dengan beberapa nilai dan cetak hasilnya.
fn latihan_3() {
    println!("--- Latihan 3: Fungsi ---");

    // TODO: panggil celcius_ke_fahrenheit dengan 0.0, 100.0, 37.0
    // dan cetak hasilnya dalam format: "[C]°C = [F]°F"
    for c in [0.0, 100.0, 37.0] {
        let f: f64 = celcius_ke_fahrenheit(c);
        println!("{}°C = {:.1}°F", c, f);
    };
    println!();
}

fn celcius_ke_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

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
