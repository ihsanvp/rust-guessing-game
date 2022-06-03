use std::{
    cmp::Ordering,
    io::{self, Write},
};

use rand::Rng;

fn main() {
    println!("GUESS THE NUMBER\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        print!("Enter your guess :");
        io::stdout().flush().expect("Flush failed");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number [0 -> 100]");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                println!("Press any button to continue.");
                io::stdin().read_line(&mut String::new()).expect("Error");
                break;
            }
        }
    }
}
