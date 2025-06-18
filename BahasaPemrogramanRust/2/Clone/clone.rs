fn main() {
    let s1 = String::from("Ghaida"); // s1 memiliki ownership atas String
    let s2 = s1.clone(); // melakukan deep copy
    println!("s1: {}, s2: {}", s1, s2);
}