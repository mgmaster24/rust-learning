use core::panic;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let val = guess.value;
        println!("You guessed: {val}");
        match val.cmp(&secret_number) {
            Ordering::Less => println!("Too Low!"),
            Ordering::Greater => println!("Too High!"),
            Ordering::Equal => {
                println!("You Win!!!! The secret number was {secret_number}.");
                break;
            }
        }
    }

    let y = false;
    if y {
        println!("this is poorly formatted");
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100. Got {}", value)
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
