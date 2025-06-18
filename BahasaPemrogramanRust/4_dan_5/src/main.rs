// // main.rs

// mod math_utils; // deklarasi modul (cari file `math_utils.rs`)

// fn main() {
//     let sum = math_utils::add(3, 4);
//     let product = math_utils::multiply(3, 4);

//     println!("Sum: {}", sum);
//     println!("Product: {}", product);
// }


// Impor crate chrono dari dependencies Cargo.toml
use chrono::Local;  // Local artinya waktu lokal (bukan UTC)

fn main() {
    // Menampilkan waktu sekarang menggunakan chrono
    println!("Waktu sekarang: {}", Local::now());

    // Informasi tambahan agar terlihat jelas saat run
    println!("Ini hasil dari cargo run!");
}
