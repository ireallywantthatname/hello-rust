use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main () {
    number_guess();
}

fn number_guess() {
    loop {
        println!("Guess the number!");

        let random_number: u32 = rand::thread_rng().gen_range(1..=100);

        println!("Please input your guess.");

        let mut guessed_number = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Error getting the number.");

        let guessed_number: u32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess {guessed_number}");

        match guessed_number.cmp(&random_number) {
            Ordering::Less => println!("Number is too low."),
            Ordering::Greater => println!("Number is too high."),
            Ordering::Equal => {
                println!("Correct guess.");
                break;
            }
        }
    }
}
