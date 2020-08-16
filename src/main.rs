use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut num_guesses: u32 = 0;

    loop {
        println!("Please input your guess (quit by entering any non-integer):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => exit(0),
        };

        num_guesses = num_guesses + 1;
        println!("You guessed: {} on attempt {}", guess, num_guesses);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
