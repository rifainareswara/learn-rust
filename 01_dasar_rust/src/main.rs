// Modul 01: Dasar Rust — Contoh
// Cara menjalankan: cargo run -p 01_dasar_rust

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║   Modul 01: Dasar Rust — Contoh      ║");
    println!("╚══════════════════════════════════════╝\n");

    demo_variables();
    demo_tipe_data();
    demo_fungsi();
    demo_control_flow();
}

// ========================================================
// 1. Variables & Mutability
// ========================================================
fn demo_variables() {
    println!("--- 1. Variables & Mutability ---");

    // Immutable by default
    let x = 5;
    println!("  x = {}", x);

    // Mutable dengan 'mut'
    let mut y = 10;
    println!("  y awal = {}", y);
    y += 5;
    println!("  y setelah y += 5 → {}", y);

    // Constant (huruf besar, tipe wajib dicantumkan)
    const TAHUN_INI: u32 = 2026;
    println!("  Tahun ini: {}", TAHUN_INI);

    // Shadowing (deklarasi ulang dengan let)
    let z = 5;
    let z = z + 1;         // z sekarang 6
    let z = z * 2;         // z sekarang 12
    println!("  z (setelah shadowing) = {}", z);

    println!();
}

// ========================================================
// 2. Tipe Data
// ========================================================
fn demo_tipe_data() {
    println!("--- 2. Tipe Data ---");

    // Scalar
    let bilangan_bulat: i32 = -42;
    let bilangan_positif: u8 = 255;
    let desimal: f64 = 3.14159;
    let benar: bool = true;
    let karakter: char = '🦀';

    println!("  i32: {}", bilangan_bulat);
    println!("  u8 max: {}", bilangan_positif);
    println!("  f64: {}", desimal);
    println!("  bool: {}", benar);
    println!("  char: {}", karakter);

    // Tuple
    let orang: (&str, i32, f64) = ("Rifai", 25, 1.75);
    println!("  Tuple — Nama: {}, Umur: {}, Tinggi: {}m", orang.0, orang.1, orang.2);

    // Array
    let bulan = ["Jan", "Feb", "Mar", "Apr", "Mei", "Jun"];
    println!("  Array — Bulan pertama: {}, jumlah: {}", bulan[0], bulan.len());

    println!();
}

// ========================================================
// 3. Fungsi
// ========================================================
fn demo_fungsi() {
    println!("--- 3. Fungsi ---");

    let hasil_kuadrat = kuadrat(9);
    println!("  Kuadrat dari 9 = {}", hasil_kuadrat);

    let (luas, keliling) = luas_keliling_persegi(5.0);
    println!("  Persegi sisi 5 → Luas: {}, Keliling: {}", luas, keliling);

    println!();
}

fn kuadrat(n: i32) -> i32 {
    n * n // ekspresi terakhir tanpa titik koma = nilai kembalian
}

fn luas_keliling_persegi(sisi: f64) -> (f64, f64) {
    let luas = sisi * sisi;
    let keliling = 4.0 * sisi;
    (luas, keliling) // tuple sebagai return value
}

// ========================================================
// 4. Control Flow
// ========================================================
fn demo_control_flow() {
    println!("--- 4. Control Flow ---");

    // if-else
    let nilai = 75;
    let grade = if nilai >= 90 {
        "A"
    } else if nilai >= 75 {
        "B"
    } else {
        "C"
    };
    println!("  Nilai {} → Grade {}", nilai, grade);

    // loop dengan break value
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("  loop berhenti di counter={}, result={}", counter, loop_result);

    // for dengan range
    print!("  for 1..=5: ");
    for n in 1..=5 {
        print!("{} ", n);
    }
    println!();

    // for dengan enumerate
    let warna = ["Merah", "Hijau", "Biru"];
    for (i, w) in warna.iter().enumerate() {
        println!("  warna[{}] = {}", i, w);
    }

    println!();
}
