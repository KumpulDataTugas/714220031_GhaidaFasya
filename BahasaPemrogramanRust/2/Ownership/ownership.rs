fn main() {
    let s = String::from("Ghaida"); // s memiliki ownership atas String
    print_string(s); // ownership dipindahkan ke fungsi
    // println!("{}", s); // ❌ error: s sudah tidak berlaku
}

fn print_string(text: String) {
    println!("Nama: {}", text);
}