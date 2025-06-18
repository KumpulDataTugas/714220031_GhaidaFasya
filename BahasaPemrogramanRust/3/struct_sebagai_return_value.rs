struct Point {
    x: i32,
    y: i32,
}

fn create_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

fn main() {
    let p = create_point(3, 4);
    println!("({}, {})", p.x, p.y);
}