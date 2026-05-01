// Modul 02: Ownership & Borrowing — Latihan
// Cara menjalankan: cargo run -p 02_ownership --bin latihan
//
// INSTRUKSI: Lengkapi setiap TODO di bawah ini.

fn main() {
    println!("=== Latihan 02: Ownership & Borrowing ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

// ── Latihan 1 ──────────────────────────────────────────
// Buat fungsi `pertama_kata(s: &String) -> &str`
// yang mengembalikan kata pertama dari sebuah kalimat.
// Petunjuk: cari index spasi pertama dengan .find(' ')
fn latihan_1() {
    println!("--- Latihan 1: String Slice ---");
    let kalimat = String::from("Belajar Rust sangat menyenangkan");

    // TODO: panggil fungsi pertama_kata dan cetak hasilnya
    // let kata = pertama_kata(&kalimat);
    // println!("Kata pertama: {}", kata);

    println!();
}

// TODO: definisikan fungsi pertama_kata di sini
// fn pertama_kata(s: &String) -> &str {
//     ...
// }

// ── Latihan 2 ──────────────────────────────────────────
// Buat fungsi `tambah_ke_vektor(v: &mut Vec<i32>, item: i32)`
// yang menambahkan item ke vektor.
// Setelah itu panggil fungsinya 3 kali, lalu cetak vektor.
fn latihan_2() {
    println!("--- Latihan 2: Mutable References ---");
    let mut angka: Vec<i32> = Vec::new();

    // TODO: panggil tambah_ke_vektor 3 kali dengan nilai berbeda
    // tambah_ke_vektor(&mut angka, 10);
    // ...

    // TODO: cetak vektor
    // println!("Vektor: {:?}", angka);

    println!();
}

// TODO: definisikan tambah_ke_vektor
// fn tambah_ke_vektor(v: &mut Vec<i32>, item: i32) {
//     ...
// }

// ── Latihan 3 ──────────────────────────────────────────
// Buat fungsi `hitung_statistik(data: &Vec<f64>) -> (f64, f64, f64)`
// yang mengembalikan (min, max, rata_rata) dari slice angka.
// CATATAN: gunakan borrowing, jangan pindahkan ownership!
fn latihan_3() {
    println!("--- Latihan 3: Borrowing + Statistik ---");
    let data = vec![4.5, 8.2, 1.1, 9.7, 3.3, 6.6, 2.0];

    // TODO: panggil hitung_statistik dan cetak hasilnya
    // let (min, max, rata) = hitung_statistik(&data);
    // println!("Min: {}, Max: {}, Rata-rata: {:.2}", min, max, rata);
    // println!("Data awal masih valid: {:?}", data); // ownership tidak berpindah!

    println!();
}

// TODO: definisikan hitung_statistik
// fn hitung_statistik(data: &Vec<f64>) -> (f64, f64, f64) {
//     ...
// }
