struct Counter {
    count: u32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let mut c = Counter { count: 0 };
    c.increment();
    println!("Count: {}", c.count);
}