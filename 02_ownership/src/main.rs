// Modul 02: Ownership & Borrowing — Contoh
// Cara menjalankan: cargo run -p 02_ownership

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║  Modul 02: Ownership & Borrowing     ║");
    println!("╚══════════════════════════════════════╝\n");

    demo_ownership();
    demo_clone();
    demo_borrowing();
    demo_mutable_references();
    demo_slices();
}

// ========================================================
// 1. Ownership & Move
// ========================================================
fn demo_ownership() {
    println!("--- 1. Ownership & Move ---");

    let s1 = String::from("hello");
    println!("  s1 = {}", s1);

    let s2 = s1; // s1 di-MOVE ke s2 → s1 tidak valid lagi
    println!("  s2 (setelah move dari s1) = {}", s2);
    // println!("{}", s1); // ← ini akan ERROR!

    // Tipe sederhana (i32, bool, dll) di-Copy, bukan di-Move
    let a = 5;
    let b = a; // COPY, bukan move
    println!("  a={} dan b={} (keduanya valid karena Copy)", a, b);

    println!();
}

// ========================================================
// 2. Clone
// ========================================================
fn demo_clone() {
    println!("--- 2. Clone ---");

    let s1 = String::from("halo dunia");
    let s2 = s1.clone(); // duplikat data di heap

    println!("  s1 = {} (masih valid!)", s1);
    println!("  s2 = {} (hasil clone)", s2);

    println!();
}

// ========================================================
// 3. Borrowing (Referensi Immutable)
// ========================================================
fn demo_borrowing() {
    println!("--- 3. Borrowing ---");

    let s = String::from("Belajar Rust");

    // Meminjam s dengan &s, s tidak berpindah ownership
    let panjang = hitung_panjang(&s);
    println!("  '{}' panjangnya {} karakter", s, panjang); // s masih valid!

    // Boleh ada banyak referensi immutable sekaligus
    let r1 = &s;
    let r2 = &s;
    println!("  r1='{}', r2='{}'", r1, r2);

    println!();
}

fn hitung_panjang(s: &String) -> usize {
    s.len()
    // s dipinjam, tidak di-drop di sini
}

// ========================================================
// 4. Mutable References
// ========================================================
fn demo_mutable_references() {
    println!("--- 4. Mutable References ---");

    let mut s = String::from("halo");
    println!("  Sebelum: {}", s);

    tambah_kata(&mut s);
    println!("  Sesudah: {}", s);

    // PENTING: hanya SATU mutable reference yang boleh ada pada satu waktu
    // let r1 = &mut s;
    // let r2 = &mut s; // ← ERROR!

    println!();
}

fn tambah_kata(s: &mut String) {
    s.push_str(", dunia!");
}

// ========================================================
// 5. Slices
// ========================================================
fn demo_slices() {
    println!("--- 5. String Slices (&str) ---");

    let kalimat = String::from("hello world rust");

    let hello = &kalimat[0..5];    // atau [..5]
    let world = &kalimat[6..11];
    let rust  = &kalimat[12..];    // sampai akhir

    println!("  Kalimat: '{}'", kalimat);
    println!("  Slice 1: '{}'", hello);
    println!("  Slice 2: '{}'", world);
    println!("  Slice 3: '{}'", rust);

    // Array slice
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[1..4]; // [20, 30, 40]
    println!("  Array slice [1..4]: {:?}", slice);

    println!();
}
