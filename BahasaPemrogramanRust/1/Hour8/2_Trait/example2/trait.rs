pub trait Show {
    // define a trait
    fn show(&self); // define a trait method
}
impl<T> Show for T
// implement the trait with generic
where
    T: ToString,
{
    // specify the String type
    fn show(&self) {
        // implement the trait method
        print!("{}", self.to_string());
    }
}
fn main() {
    String::from("C# in 8 Hours").show(); // call method
}
