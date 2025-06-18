fn main() {
    let x = 5;
    match x {
        var @ 2...6 => println!("{}", var), // binding
        _ => println!("others"),
    }
}
