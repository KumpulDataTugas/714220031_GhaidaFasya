fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5]; // ambil slice "hello"
    println!("Slice: {}", hello);
}