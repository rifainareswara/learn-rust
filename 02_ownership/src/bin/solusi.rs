// Modul 02: Ownership & Borrowing — Solusi
// Cara menjalankan: cargo run -p 02_ownership --bin solusi

fn main() {
    println!("=== Solusi Latihan 02: Ownership & Borrowing ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

fn latihan_1() {
    println!("--- Latihan 1: String Slice ---");
    let kalimat = String::from("Belajar Rust sangat menyenangkan");
    let kata = pertama_kata(&kalimat);
    println!("Kalimat: '{}'", kalimat);
    println!("Kata pertama: '{}'", kata);
    println!();
}

fn pertama_kata(s: &String) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s.as_str(),
    }
}

fn latihan_2() {
    println!("--- Latihan 2: Mutable References ---");
    let mut angka: Vec<i32> = Vec::new();
    tambah_ke_vektor(&mut angka, 10);
    tambah_ke_vektor(&mut angka, 20);
    tambah_ke_vektor(&mut angka, 30);
    println!("Vektor: {:?}", angka);
    println!();
}

fn tambah_ke_vektor(v: &mut Vec<i32>, item: i32) {
    v.push(item);
}

fn latihan_3() {
    println!("--- Latihan 3: Borrowing + Statistik ---");
    let data = vec![4.5, 8.2, 1.1, 9.7, 3.3, 6.6, 2.0];
    let (min, max, rata) = hitung_statistik(&data);
    println!("Data: {:?}", data);
    println!("Min: {:.1}, Max: {:.1}, Rata-rata: {:.2}", min, max, rata);
    println!("Data asal masih valid: {} elemen", data.len());
    println!();
}

fn hitung_statistik(data: &Vec<f64>) -> (f64, f64, f64) {
    let mut min = data[0];
    let mut max = data[0];
    let mut sum = 0.0;
    for &n in data {
        if n < min { min = n; }
        if n > max { max = n; }
        sum += n;
    }
    let rata = sum / data.len() as f64;
    (min, max, rata)
}
