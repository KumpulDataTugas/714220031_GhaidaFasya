struct Square {
    // create a struct
    len: i32,
    wid: i32,
}
fn main() {
    let table = Square { len: 10, wid: 8 }; // initialization
    println!("The area is {}", table.len * table.wid); // access
}
