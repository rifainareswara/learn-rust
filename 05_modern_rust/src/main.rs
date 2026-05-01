// Modul 05: Modern Rust — Contoh
// Cara menjalankan: cargo run -p 05_modern_rust

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum AppError {
    BagiDenganNol,
    InputTidakValid(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::BagiDenganNol => write!(f, "Tidak bisa membagi dengan nol!"),
            AppError::InputTidakValid(msg) => write!(f, "Input tidak valid: {}", msg),
        }
    }
}

fn bagi(a: f64, b: f64) -> Result<f64, AppError> {
    if b == 0.0 {
        Err(AppError::BagiDenganNol)
    } else {
        Ok(a / b)
    }
}

fn parse_dan_bagi(a: &str, b: &str) -> Result<f64, AppError> {
    let x: f64 = a.parse().map_err(|_| AppError::InputTidakValid(format!("'{}' bukan angka", a)))?;
    let y: f64 = b.parse().map_err(|_| AppError::InputTidakValid(format!("'{}' bukan angka", b)))?;
    bagi(x, y)
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Modul 05: Modern Rust               ║");
    println!("╚══════════════════════════════════════╝\n");

    // 1. Error Handling
    println!("--- 1. Error Handling ---");
    for (a, b) in [(10.0, 2.0), (5.0, 0.0)] {
        match bagi(a, b) {
            Ok(h) => println!("  {} / {} = {}", a, b, h),
            Err(e) => println!("  {} / {} → Error: {}", a, b, e),
        }
    }
    for (a, b) in [("20", "4"), ("abc", "2")] {
        match parse_dan_bagi(a, b) {
            Ok(h) => println!("  parse('{}','{}') = {}", a, b, h),
            Err(e) => println!("  parse('{}','{}') → {}", a, b, e),
        }
    }
    println!();

    // 2. Smart Pointers
    println!("--- 2. Smart Pointers ---");
    let kotak = Box::new(42);
    println!("  Box<i32> = {}", kotak);

    let data = Rc::new(String::from("Milik Bersama"));
    let owner1 = Rc::clone(&data);
    println!("  Rc owner count: {}", Rc::strong_count(&data));
    println!("  owner1: {}", owner1);
    drop(owner1);
    println!("  Setelah drop: count = {}", Rc::strong_count(&data));
    println!();

    // 3. Concurrency dengan Arc + Mutex
    println!("--- 3. Concurrency ---");
    let counter = Arc::new(Mutex::new(0i32));
    let mut handles = vec![];

    for i in 0..5 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1;
            println!("  Thread {} → counter = {}", i, *num);
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("  ✓ Counter akhir: {}", *counter.lock().unwrap());
    println!();

    println!("✅ Modern Rust selesai!");
}
