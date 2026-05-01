// Modul 04: Intermediate Rust — Solusi
// Cara menjalankan: cargo run -p 04_intermediate --bin solusi

use std::collections::HashMap;

fn main() {
    println!("=== Solusi Latihan 04: Intermediate Rust ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

// --- Trait & Generics ---
trait Hitung {
    fn luas(&self) -> f64;
    fn keliling(&self) -> f64;
}

struct Lingkaran { jari_jari: f64 }
struct Persegi { sisi: f64 }

impl Hitung for Lingkaran {
    fn luas(&self) -> f64 { std::f64::consts::PI * self.jari_jari * self.jari_jari }
    fn keliling(&self) -> f64 { 2.0 * std::f64::consts::PI * self.jari_jari }
}

impl Hitung for Persegi {
    fn luas(&self) -> f64 { self.sisi * self.sisi }
    fn keliling(&self) -> f64 { 4.0 * self.sisi }
}

fn cetak_info(nama: &str, bentuk: &impl Hitung) {
    println!("  {} → Luas: {:.2}, Keliling: {:.2}", nama, bentuk.luas(), bentuk.keliling());
}

fn latihan_1() {
    println!("--- Latihan 1: Traits & Generics ---");
    let l = Lingkaran { jari_jari: 5.0 };
    let p = Persegi { sisi: 4.0 };
    cetak_info("Lingkaran (r=5)", &l);
    cetak_info("Persegi (s=4)", &p);
    println!();
}

fn latihan_2() {
    println!("--- Latihan 2: Vec & Iterators ---");
    let buah = vec!["apel", "mangga", "durian", "jeruk", "semangka"];

    // (a) cetak semua huruf kapital
    println!("  Huruf kapital:");
    for b in &buah {
        println!("    {}", b.to_uppercase());
    }

    // (b) filter panjang > 5
    let panjang: Vec<&&str> = buah.iter().filter(|b| b.len() > 5).collect();
    println!("  Nama > 5 huruf: {:?}", panjang);

    // (c) total karakter
    let total: usize = buah.iter().map(|b| b.len()).sum();
    println!("  Total karakter: {}", total);
    println!();
}

fn latihan_3() {
    println!("--- Latihan 3: HashMap ---");
    let mut nilai: HashMap<String, Vec<u32>> = HashMap::new();
    nilai.insert(String::from("Rifai"), vec![80, 90, 85]);
    nilai.insert(String::from("Budi"), vec![70, 75, 80]);
    nilai.insert(String::from("Siti"), vec![95, 92, 98]);

    // (a) rata-rata tiap mahasiswa
    let mut rata_rata: Vec<(String, f64)> = nilai
        .iter()
        .map(|(nama, nilais)| {
            let sum: u32 = nilais.iter().sum();
            let avg = sum as f64 / nilais.len() as f64;
            (nama.clone(), avg)
        })
        .collect();

    rata_rata.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (nama, avg) in &rata_rata {
        println!("  {}: {:.1}", nama, avg);
    }

    // (b) mahasiswa tertinggi
    if let Some((nama, avg)) = rata_rata.first() {
        println!("  🏆 Tertinggi: {} dengan rata-rata {:.1}", nama, avg);
    }
    println!();
}
