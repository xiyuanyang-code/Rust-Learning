use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
fn main() {
    println!("Hello world!");
    // panic!("Oh No! It will crash");

    // let v = vec![1, 23, 4, 5, 5];
    // this will cause error, but not a compile error
    // index out of bounds: the len is 5 but the index is 100
    // println!("{}", v[100]);
    test_file_open();
    test_test_file_open();
    test_test_file_open_optimized();
    unwrap_test();
    expect_test();
    let result = read_username_from_file();
    println!("{result:?}");
    let result = read_username_from_file_new();
    println!("{result:?}");
    let result = read_username_from_file_new_new();
    println!("{result:?}");
    let result = read_username_from_file_new_new_new();
    println!("{result:?}");
}

fn test_file_open() {
    let result = File::open("./README.md");
    println!("{result:?}");
}

fn test_test_file_open() {
    let file_list = vec!["./README.md", "./readme.md"];
    for file_name in &file_list {
        println!("Trying to read {}...", file_name);
        let result = File::open(&file_name);
        let greeting_file = match result {
            Ok(file) => {
                println!("Loading file successfully!");
                file
            }
            Err(error) => {
                println!("Error! Trying to create a new file");
                match error.kind() {
                    ErrorKind::NotFound => match File::create(&file_name) {
                        Ok(fc) => fc,
                        Err(error) => panic!("Error creating file: {error:?}"),
                    },
                    _ => {
                        panic!("Other problems")
                    }
                }
            }
        };
        println!("{greeting_file:?}");
    }
}

fn test_test_file_open_optimized() {
    let file_list = vec!["./README.md", "./readme.md"];

    for file_name in &file_list {
        println!("Trying to read {}...", file_name);

        let Ok(file) = File::open(file_name) else {
            // This block executes if File::open returns an `Err`
            // You can also add more complex logic here.
            println!("Error! Trying to create a new file");

            // Re-attempt to open the file to get the specific error kind.
            let error = File::open(file_name).unwrap_err();
            if error.kind() == ErrorKind::NotFound {
                // If it's a NotFound error, try creating the file.
                File::create(file_name).unwrap_or_else(|e| {
                    panic!("Error creating file: {e:?}");
                });

                // Continue to the next file in the loop.
                continue;
            } else {
                // For all other errors, panic.
                panic!("Other problems: {:?}", error);
            }
        };
        println!("{file:?}");

        // This code only runs if File::open was successful.
        println!("Loading file successfully!");
    }
}

fn unwrap_test() {
    let file = File::open("./rEADME.md").unwrap();
    println!("{file:?}");
}

fn expect_test() {
    let file = File::open("./rEADME.md").expect("Error, this is a test panic message");
    println!("{file:?}");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // this function will read the name (content) from the given file, then return a result type
    let username_file_result = File::open("README.md");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    // fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize> {
    //     (&*self).read_to_string(buf)
    // }
    // read string from files
}

fn read_username_from_file_new() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_new_new() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("./README.md")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_new_new_new() -> Result<String, io::Error> {
    fs::read_to_string("./README.md")
}
