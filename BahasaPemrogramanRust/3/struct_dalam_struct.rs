struct Address {
    city: String,
    zip: u32,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let addr = Address {
        city: String::from("Bandung"),
        zip: 40111,
    };
    let person = Person {
        name: String::from("Budi"),
        address: addr,
    };
    println!("{} lives in {}", person.name, person.address.city);
}