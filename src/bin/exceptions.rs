use std::{fs::File, io::ErrorKind};
fn main() {
    println!("Hello world!");
    // panic!("Oh No! It will crash");

    // let v = vec![1, 23, 4, 5, 5];
    // this will cause error, but not a compile error
    // index out of bounds: the len is 5 but the index is 100
    // println!("{}", v[100]);
    // test_file_open();
    // test_test_file_open();
    // test_test_file_open_optimized();
    // unwrap_test();
    expect_test();
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

        // This code only runs if File::open was successful.
        println!("Loading file successfully!");
    }
}

fn unwrap_test() {
    let file = File::open("./rEADME.md").unwrap();
}

fn expect_test() {
    let file = File::open("./rEADME.md").expect("Error, this is a test panic message");
}
