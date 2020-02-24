use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();  // default (let guess = ...) would be immutable

        // Calling expect won't recover from a potentially error, instead will raise and crash.
        // When no error, returns number of bytes written
        let bytes_written = io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // parse() is a casting func
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {} ({} bytes)", guess, bytes_written);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
