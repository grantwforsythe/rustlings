use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess a number between 1 and 100!");

    game_loop()
}

fn game_loop() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Variable shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Winner!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
