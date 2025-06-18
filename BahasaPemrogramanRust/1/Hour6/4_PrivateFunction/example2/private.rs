mod my_module {
    pub fn a() {
        println!("function a");
        b(); // call a private function b
    }
    fn b() {
        // function b is private
        println!("function b");
    }
}
fn main() {
    my_module::a();
}
