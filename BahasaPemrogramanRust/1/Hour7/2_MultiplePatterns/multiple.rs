fn main() {
    let num = 3;
    match num {
        1 => println!("one"),
        2 | 3 => println!("two or three"), // multiple patterns
        _ => println!("others"),
    }
}
