#[derive(Default)]
struct Config {
    debug: bool,
    version: String,
}

fn main() {
    let config = Config {
        version: String::from("1.0"),
        ..Default::default()
    };
    println!("Version: {}, Debug: {}", config.version, config.debug);
}