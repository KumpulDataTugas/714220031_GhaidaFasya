struct Info {
    message: String,
}

enum Status {
    Success(Info),
    Error(String),
}

fn print_status(s: Status) {
    match s {
        Status::Success(info) => println!("Success: {}", info.message),
        Status::Error(err) => println!("Error: {}", err),
    }
}

fn main() {
    let success = Status::Success(Info {
        message: String::from("All good!"),
    });
    print_status(success);
}