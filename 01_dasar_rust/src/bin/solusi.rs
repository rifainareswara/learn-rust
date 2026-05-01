// Modul 01: Dasar Rust — Solusi
// Cara menjalankan: cargo run -p 01_dasar_rust --bin solusi

fn main() {
    println!("=== Solusi Latihan 01: Dasar Rust ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
    latihan_4();
}

fn latihan_1() {
    println!("--- Latihan 1: Variables ---");
    let nama = "Rifai";
    let mut umur = 25;
    umur += 1;
    println!("Nama: {}, Umur: {}", nama, umur);
    println!();
}

fn latihan_2() {
    println!("--- Latihan 2: Tuple ---");
    let jakarta: (&str, u32, f64) = ("Jakarta", 10_500_000, 664.01);
    println!("Kota: {}, Populasi: {}, Luas: {} km²", jakarta.0, jakarta.1, jakarta.2);
    println!();
}

fn latihan_3() {
    println!("--- Latihan 3: Fungsi ---");
    for c in [0.0, 100.0, 37.0] {
        let f = celcius_ke_fahrenheit(c);
        println!("{}°C = {:.1}°F", c, f);
    }
    println!();
}

fn celcius_ke_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn latihan_4() {
    println!("--- Latihan 4: FizzBuzz ---");
    for n in 1..=10 {
        let output = if n % 15 == 0 {
            String::from("FizzBuzz")
        } else if n % 3 == 0 {
            String::from("Fizz")
        } else if n % 5 == 0 {
            String::from("Buzz")
        } else {
            n.to_string()
        };
        println!("{}", output);
    }
    println!();
}
