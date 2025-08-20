use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello World, this is a guessing number game");

    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read lines");

        if guess.trim() == "quit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error, please type a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Exactly the same!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
