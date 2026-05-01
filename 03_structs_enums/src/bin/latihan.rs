// Modul 03: Structs & Enums — Latihan
// Cara menjalankan: cargo run -p 03_structs_enums --bin latihan

fn main() {
    println!("=== Latihan 03: Structs & Enums ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

// ── Latihan 1 ──────────────────────────────────────────
// Buat struct `Buku` dengan field: judul (String), penulis (String), halaman (u32)
// Implementasikan method `info(&self)` yang mencetak info buku,
// dan associated function `baru(...)` sebagai constructor.
fn latihan_1() {
    println!("--- Latihan 1: Struct Buku ---");

    // TODO: definisikan struct Buku dan impl-nya

    // TODO: buat 2 buku menggunakan Buku::baru(...)
    // dan panggil .info() untuk masing-masing

    println!();
}

// ── Latihan 2 ──────────────────────────────────────────
// Buat enum `StatusPengiriman` dengan varian:
//   Menunggu, DikirimKe(String), Tiba, Dibatalkan(String)
// Buat fungsi `cek_status(status: &StatusPengiriman)` yang mencetak pesan sesuai varian.
fn latihan_2() {
    println!("--- Latihan 2: Enum StatusPengiriman ---");

    // TODO: definisikan enum StatusPengiriman

    // TODO: buat beberapa status dan panggil cek_status
    // let s1 = StatusPengiriman::Menunggu;
    // let s2 = StatusPengiriman::DikirimKe(String::from("Bandung"));
    // let s3 = StatusPengiriman::Tiba;
    // let s4 = StatusPengiriman::Dibatalkan(String::from("Stok habis"));

    println!();
}

// TODO: fn cek_status(status: &StatusPengiriman) { ... }

// ── Latihan 3 ──────────────────────────────────────────
// Buat fungsi `bagi_aman(a: i32, b: i32) -> Option<i32>`
// yang mengembalikan Some(hasil) jika b != 0, atau None jika b == 0.
// Panggil dengan beberapa kasus dan cetak hasilnya.
fn latihan_3() {
    println!("--- Latihan 3: Option<T> ---");

    // TODO: panggil bagi_aman dengan (10, 2), (10, 0), (7, 3)
    // dan cetak hasilnya menggunakan match

    println!();
}

// TODO: fn bagi_aman(a: i32, b: i32) -> Option<i32> { ... }
