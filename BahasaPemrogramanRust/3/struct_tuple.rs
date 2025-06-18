struct Color(u8, u8, u8);

fn print_color(c: &Color) {
    println!("RGB: {}, {}, {}", c.0, c.1, c.2);
}

fn main() {
    let red = Color(255, 0, 0);
    print_color(&red);
}