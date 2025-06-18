struct Member {
    // create a struct
    id: i32, // member: type
    name: String,
    working: bool,
}
fn main() {
    let clerk = Member {
        // initialize the struct
        id: 717220031, // member: value
        name: "Ghaida".to_string(),
        working: true,
    };
    println!("ID is {}", clerk.id); // access the members
    println!("Name is {}", clerk.name);
    println!("Working is {}", clerk.working);
}
