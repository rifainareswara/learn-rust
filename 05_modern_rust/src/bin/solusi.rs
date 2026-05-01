// Modul 05: Modern Rust — Solusi
// Cara menjalankan: cargo run -p 05_modern_rust --bin solusi

use std::sync::{Arc, Mutex};
use std::thread;

// --- Custom Error ---
#[derive(Debug)]
enum KalkulatorError {
    BagiNol,
    AkarNegatif,
}

impl std::fmt::Display for KalkulatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KalkulatorError::BagiNol => write!(f, "Error: tidak bisa membagi dengan nol"),
            KalkulatorError::AkarNegatif => write!(f, "Error: tidak bisa mengambil akar bilangan negatif"),
        }
    }
}

fn bagi(a: f64, b: f64) -> Result<f64, KalkulatorError> {
    if b == 0.0 { Err(KalkulatorError::BagiNol) } else { Ok(a / b) }
}

fn akar(n: f64) -> Result<f64, KalkulatorError> {
    if n < 0.0 { Err(KalkulatorError::AkarNegatif) } else { Ok(n.sqrt()) }
}

fn hitung(a: f64, b: f64) -> Result<f64, KalkulatorError> {
    let hasil_bagi = bagi(a, b)?;  // jika Err, langsung return
    let hasil_akar = akar(hasil_bagi)?;
    Ok(hasil_akar)
}

fn main() {
    println!("=== Solusi Latihan 05: Modern Rust ===\n");
    latihan_1();
    latihan_2();
}

fn latihan_1() {
    println!("--- Latihan 1: Custom Error & Operator ? ---");
    let kasus = [(16.0, 4.0), (9.0, 0.0), (-4.0, 1.0)];
    for (a, b) in kasus {
        match hitung(a, b) {
            Ok(h) => println!("  hitung({}, {}) = {:.4}", a, b, h),
            Err(e) => println!("  hitung({}, {}) → {}", a, b, e),
        }
    }
    println!();
}

fn latihan_2() {
    println!("--- Latihan 2: Arc + Mutex + Threads ---");
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    let akhir = *counter.lock().unwrap();
    println!("  Counter akhir: {} (harus 1000)", akhir);
    assert_eq!(akhir, 1000, "Counter harus 1000!");
    println!("  ✅ Berhasil!");
    println!();
}
