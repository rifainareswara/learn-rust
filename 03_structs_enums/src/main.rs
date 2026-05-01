// Modul 03: Structs & Enums — Contoh
// Cara menjalankan: cargo run -p 03_structs_enums

// ========================================================
// 1. Classic Struct
// ========================================================
struct User {
    username: String,
    email: String,
    active: bool,
    login_count: u32,
}

impl User {
    // Associated function (constructor)
    fn baru(username: String, email: String) -> User {
        User {
            username,  // field init shorthand
            email,
            active: true,
            login_count: 0,
        }
    }

    // Method
    fn perkenalan(&self) {
        println!("  Halo! Saya {} ({})", self.username, self.email);
    }

    fn login(&mut self) {
        self.login_count += 1;
        println!("  {} login ke-{}", self.username, self.login_count);
    }
}

// ========================================================
// 2. Tuple Struct
// ========================================================
struct Warna(u8, u8, u8); // RGB

impl Warna {
    fn tampilkan(&self) {
        println!("  RGB({}, {}, {})", self.0, self.1, self.2);
    }
}

// ========================================================
// 3. Enums dengan berbagai varian
// ========================================================
enum Bentuk {
    Lingkaran(f64),             // jari-jari
    Persegi(f64),               // sisi
    Persegipanjang(f64, f64),   // lebar, tinggi
    Titik,                      // tanpa data
}

impl Bentuk {
    fn luas(&self) -> f64 {
        match self {
            Bentuk::Lingkaran(r) => std::f64::consts::PI * r * r,
            Bentuk::Persegi(s) => s * s,
            Bentuk::Persegipanjang(l, t) => l * t,
            Bentuk::Titik => 0.0,
        }
    }

    fn nama(&self) -> &str {
        match self {
            Bentuk::Lingkaran(_) => "Lingkaran",
            Bentuk::Persegi(_) => "Persegi",
            Bentuk::Persegipanjang(_, _) => "Persegipanjang",
            Bentuk::Titik => "Titik",
        }
    }
}

// ========================================================
// 4. Option<T>
// ========================================================
fn cari_user(nama: &str) -> Option<String> {
    let database = vec!["rifai", "budi", "siti"];
    if database.contains(&nama) {
        Some(format!("User '{}' ditemukan!", nama))
    } else {
        None
    }
}

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Modul 03: Structs & Enums           ║");
    println!("╚══════════════════════════════════════╝\n");

    // --- Struct ---
    println!("--- 1. Classic Struct ---");
    let mut user = User::baru(
        String::from("rifai"),
        String::from("rifai@example.com"),
    );
    user.perkenalan();
    user.login();
    user.login();

    // Struct update syntax
    let user2 = User {
        username: String::from("budi"),
        email: String::from("budi@example.com"),
        ..user  // ambil sisa field dari user
    };
    user2.perkenalan();
    println!();

    // --- Tuple Struct ---
    println!("--- 2. Tuple Struct ---");
    let merah = Warna(255, 0, 0);
    let hijau = Warna(0, 255, 0);
    print!("  Merah: "); merah.tampilkan();
    print!("  Hijau: "); hijau.tampilkan();
    println!();

    // --- Enums & Pattern Matching ---
    println!("--- 3. Enums & Pattern Matching ---");
    let bentuk_bentuk = vec![
        Bentuk::Lingkaran(5.0),
        Bentuk::Persegi(4.0),
        Bentuk::Persegipanjang(6.0, 3.0),
        Bentuk::Titik,
    ];
    for b in &bentuk_bentuk {
        println!("  {} → luas: {:.2}", b.nama(), b.luas());
    }
    println!();

    // --- Option<T> ---
    println!("--- 4. Option<T> ---");
    let nama_cari = "rifai";
    match cari_user(nama_cari) {
        Some(pesan) => println!("  ✓ {}", pesan),
        None => println!("  ✗ User '{}' tidak ditemukan.", nama_cari),
    }

    // if let — lebih ringkas untuk satu kasus
    if let Some(pesan) = cari_user("admin") {
        println!("  ✓ {}", pesan);
    } else {
        println!("  ✗ User 'admin' tidak ada.");
    }

    println!();
}
