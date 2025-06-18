enum Language {
    // define an enum
    JS, // member
    GO,
    VB,
}
fn program(var: Language) {
    match var {
        // using match statement
        Language::JS => println!("JS in 8 Hours"),
        Language::GO => println!("GO in 8 Hours"),
        Language::VB => println!("VB in 8 Hours"),
    }
}
fn main() {
    program(Language::JS); // access the member
    program(Language::GO);
    program(Language::VB);
}
