mod my_module {
    pub fn a() {
        // function is public
        println!("function a");
    }
    fn b() {
        // function b is private
        println!("function b");
    }
}
fn main() {
    my_module::a();
    my_module::b(); // call a private function
}
