fn main() {
let num = funt(100); // calls the function
println!("The value of num is: { }", num);
}
fn funt(num: i32) -> i32 { // specify a return type
num + 200 // return a value to the caller
}