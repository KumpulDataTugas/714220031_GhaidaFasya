fn main() {
    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = |c: char| capacity.push(c); // closure
        my_closure('G'); // call the closure
    }
    println!("{:?}", capacity); // {:?} is used to output a string
}
