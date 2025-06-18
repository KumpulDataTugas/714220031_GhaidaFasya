struct Pair<T> {
    first: T,
    second: T,
}

fn print_pair<T: std::fmt::Debug>(p: &Pair<T>) {
    println!("Pair: {:?}, {:?}", p.first, p.second);
}

fn main() {
    let pair = Pair { first: 1, second: 2 };
    print_pair(&pair);
}