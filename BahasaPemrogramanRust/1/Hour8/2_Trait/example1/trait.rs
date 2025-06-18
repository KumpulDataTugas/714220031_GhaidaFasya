struct Circle {
    // create a struct type
    radius: f32, // struct member
}
trait Calculate {
    // define a trait
    fn area(&self) -> f32; // define a trait method
}
impl Calculate for Circle {
    // implement the trait
    fn area(&self) -> f32 {
        // implement the trait method
        std::f32::consts::PI * self.radius * self.radius
    }
}
fn main() {
    let obj = Circle { radius: 2000.00 }; // create a struct object
    println!("The Circle area is: {}", obj.area()); // call the method
}
