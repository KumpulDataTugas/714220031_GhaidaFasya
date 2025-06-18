fn foo() -> bool { // specify a return type
return true // return a value to the caller
}
fn main() {
let b = foo(); // foo() is a caller
println!("The result is: { }", b);
}