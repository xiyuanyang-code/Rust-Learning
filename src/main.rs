enum ContentAvailable {
    Strings,
    Enums,
}

fn main() {
    println!("Hello, welcome to the world of rust");
    println!("This is the recording for my learning maps for Rust Learning");
    println!("For more info, visit README.md!");
    println!("Enjoy your stay here!");
    println!("============================================");

    println!("Code available: ");
    list_available_binaries();
}

fn list_available_binaries() {
    match ContentAvailable::Strings {
        ContentAvailable::Strings => println!("- strings"),
        _ => {}
    }
    
    match ContentAvailable::Enums {
        ContentAvailable::Enums => println!("- enums"),
        _ => {}
    }
}