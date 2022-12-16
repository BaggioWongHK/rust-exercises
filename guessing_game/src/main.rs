use std::io;
use rand::Rng;

fn main() {
    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let trimmed = guess.trim();

        let guess = match trimmed.parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("{trimmed} is not a number. Try again with a number between 1 - 100.\n");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Equal => {
                println!("You're right!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big!"),
        }

        println!("The secret number is {secret_number}\n");
    }
}
