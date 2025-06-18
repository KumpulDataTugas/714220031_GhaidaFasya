fn main(){
let a = [0, 10, 20, 30, 40, 50, 60]; // create an array
let slice = &a[2..5]; // extract from a[2] to a[4]
println!("{}",slice[0] ); // show slice’s elements
println!("{}",slice[1] );
println!("{}",slice[2] );
}