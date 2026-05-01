// Modul 04: Intermediate Rust — Contoh
// Cara menjalankan: cargo run -p 04_intermediate

use std::collections::HashMap;

// ========================================================
// 1. Traits
// ========================================================
trait Ringkasan {
    fn ringkas(&self) -> String;

    // Default implementation
    fn pratinjau(&self) -> String {
        format!("[Pratinjau] {}...", &self.ringkas()[..20.min(self.ringkas().len())])
    }
}

struct Artikel {
    judul: String,
    penulis: String,
    konten: String,
}

impl Ringkasan for Artikel {
    fn ringkas(&self) -> String {
        format!("'{}' oleh {}", self.judul, self.penulis)
    }
}

struct Tweet {
    akun: String,
    pesan: String,
}

impl Ringkasan for Tweet {
    fn ringkas(&self) -> String {
        format!("@{}: {}", self.akun, self.pesan)
    }
}

// ========================================================
// 2. Generics
// ========================================================
fn tampilkan_ringkasan<T: Ringkasan>(item: &T) {
    println!("  📰 {}", item.ringkas());
}

// Dengan 'impl Trait' — lebih ringkas
fn tampilkan_pratinjau(item: &impl Ringkasan) {
    println!("  👁  {}", item.pratinjau());
}

// Generic struct
struct Pasangan<T> {
    pertama: T,
    kedua: T,
}

impl<T: std::fmt::Display + PartialOrd> Pasangan<T> {
    fn baru(a: T, b: T) -> Self {
        Pasangan { pertama: a, kedua: b }
    }
    fn yang_terbesar(&self) -> &T {
        if self.pertama > self.kedua { &self.pertama } else { &self.kedua }
    }
}

// ========================================================
// 3. Lifetimes
// ========================================================
fn terpanjang<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Modul 04: Intermediate Rust         ║");
    println!("╚══════════════════════════════════════╝\n");

    // --- Traits & Generics ---
    println!("--- 1. Traits & Generics ---");
    let artikel = Artikel {
        judul: String::from("Rust Bahasa Masa Depan"),
        penulis: String::from("Rifai"),
        konten: String::from("Rust adalah bahasa yang cepat dan aman..."),
    };
    let tweet = Tweet {
        akun: String::from("rifai_dev"),
        pesan: String::from("Belajar Rust itu seru banget!"),
    };

    tampilkan_ringkasan(&artikel);
    tampilkan_ringkasan(&tweet);
    tampilkan_pratinjau(&artikel);
    println!();

    // --- Generic Struct ---
    println!("--- 2. Generic Struct ---");
    let pasangan_angka = Pasangan::baru(30, 50);
    let pasangan_kata = Pasangan::baru("apel", "zebra");
    println!("  Terbesar dari (30, 50): {}", pasangan_angka.yang_terbesar());
    println!("  Terbesar dari (apel, zebra): {}", pasangan_kata.yang_terbesar());
    println!();

    // --- Vec<T> ---
    println!("--- 3. Vec<T> ---");
    let mut nilai: Vec<i32> = Vec::new();
    nilai.push(85);
    nilai.push(92);
    nilai.push(78);
    nilai.push(95);

    let jumlah: i32 = nilai.iter().sum();
    let rata_rata = jumlah as f64 / nilai.len() as f64;
    println!("  Nilai: {:?}", nilai);
    println!("  Rata-rata: {:.1}", rata_rata);

    // Filter nilai di atas 80
    let lulus: Vec<&i32> = nilai.iter().filter(|&&n| n > 80).collect();
    println!("  Di atas 80: {:?}", lulus);
    println!();

    // --- HashMap<K, V> ---
    println!("--- 4. HashMap<K, V> ---");
    let mut inventori: HashMap<String, u32> = HashMap::new();
    inventori.insert(String::from("Apel"), 50);
    inventori.insert(String::from("Jeruk"), 30);
    inventori.insert(String::from("Mangga"), 25);

    // Entry API — tambah hanya jika belum ada
    inventori.entry(String::from("Semangka")).or_insert(100);
    inventori.entry(String::from("Apel")).or_insert(999); // tidak berubah, sudah ada

    for (barang, jumlah) in &inventori {
        println!("  {} → {} buah", barang, jumlah);
    }
    println!();

    // --- Lifetimes ---
    println!("--- 5. Lifetimes ---");
    let teks1 = String::from("kalimat panjang sekali");
    let hasil;
    {
        let teks2 = String::from("pendek");
        hasil = terpanjang(teks1.as_str(), teks2.as_str());
        println!("  Yang terpanjang: '{}'", hasil);
    }
    println!();
}
