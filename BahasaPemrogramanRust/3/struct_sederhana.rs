struct User {
    name: String,
    age: u8,
}

fn print_user(user: &User) {
    println!("Name: {}, Age: {}", user.name, user.age);
}

fn main() {
    let user = User {
        name: String::from("Alice"),
        age: 20,
    };
    print_user(&user);
}