fn main() {
    {
        let scoped = String::from("in scope");
        println!("{}", scoped); // hanya hidup dalam blok ini
    }
    // println!("{}", scoped); // ❌ error: scoped sudah out of scope
}