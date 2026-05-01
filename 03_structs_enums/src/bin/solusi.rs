// Modul 03: Structs & Enums — Solusi
// Cara menjalankan: cargo run -p 03_structs_enums --bin solusi

fn main() {
    println!("=== Solusi Latihan 03: Structs & Enums ===\n");
    latihan_1();
    latihan_2();
    latihan_3();
}

// --- Struct ---
struct Buku {
    judul: String,
    penulis: String,
    halaman: u32,
}

impl Buku {
    fn baru(judul: &str, penulis: &str, halaman: u32) -> Buku {
        Buku {
            judul: String::from(judul),
            penulis: String::from(penulis),
            halaman,
        }
    }

    fn info(&self) {
        println!("  📖 '{}' oleh {} ({} hal.)", self.judul, self.penulis, self.halaman);
    }
}

fn latihan_1() {
    println!("--- Latihan 1: Struct Buku ---");
    let b1 = Buku::baru("The Rust Programming Language", "Steve Klabnik", 526);
    let b2 = Buku::baru("Programming Rust", "Jim Blandy", 622);
    b1.info();
    b2.info();
    println!();
}

// --- Enum ---
enum StatusPengiriman {
    Menunggu,
    DikirimKe(String),
    Tiba,
    Dibatalkan(String),
}

fn cek_status(status: &StatusPengiriman) {
    match status {
        StatusPengiriman::Menunggu => println!("  ⏳ Paket sedang menunggu diproses"),
        StatusPengiriman::DikirimKe(kota) => println!("  🚚 Paket dalam perjalanan ke {}", kota),
        StatusPengiriman::Tiba => println!("  ✅ Paket telah tiba!"),
        StatusPengiriman::Dibatalkan(alasan) => println!("  ❌ Dibatalkan: {}", alasan),
    }
}

fn latihan_2() {
    println!("--- Latihan 2: Enum StatusPengiriman ---");
    let statuses = vec![
        StatusPengiriman::Menunggu,
        StatusPengiriman::DikirimKe(String::from("Bandung")),
        StatusPengiriman::Tiba,
        StatusPengiriman::Dibatalkan(String::from("Stok habis")),
    ];
    for s in &statuses {
        cek_status(s);
    }
    println!();
}

// --- Option ---
fn bagi_aman(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn latihan_3() {
    println!("--- Latihan 3: Option<T> ---");
    for (a, b) in [(10, 2), (10, 0), (7, 3)] {
        match bagi_aman(a, b) {
            Some(hasil) => println!("  {} / {} = {}", a, b, hasil),
            None => println!("  {} / {} → tidak bisa (pembagi nol)", a, b),
        }
    }
    println!();
}
