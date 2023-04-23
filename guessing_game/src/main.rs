use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing game!");
    println!("Guess the number!");
    println!("Please insert the number:");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess_number: String = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read line");

        println!("Your number: {guess_number}");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
