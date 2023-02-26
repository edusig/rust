use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    let mut secret = rand::thread_rng().gen_range(1..101);
    let mut tries_left: i8 = 8;

    loop {
        match tries_left {
            -1 => println!("You win!"),
            0 => println!("You lost!"),
            _ => println!("You have {} tries left", tries_left),
        }

        if tries_left <= 0 {
            println!("Wanna play again? (Y/n)");
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Failed to read line");
            match play_again.trim() {
                "n" | "N" => break,
                "Y" | "y" | "" => {
                    println!("Guess the number!");
                    secret = rand::thread_rng().gen_range(1..101);
                    tries_left = 8;
                }
                _ => continue,
            }
        }

        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        tries_left -= 1;

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                tries_left = -1;
            }
        }
    }
}
