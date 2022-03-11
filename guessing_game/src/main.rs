use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 1 and 100:");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error: {}.", error);
                continue;
            }
        };

        println!("Your guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The secret number is: {}.", secret_number);
}
