fn main() {
    let s = String::from("Rust");
    print_reference(&s); // meminjam (borrow) isi dari s
    println!("{}", s); // ✅ masih bisa dipakai
}

fn print_reference(s: &String) {
    println!("Reference: {}", s);
}