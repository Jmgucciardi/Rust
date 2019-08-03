use std::io::{stdin};
use std::cmp::Ordering;
use rand::Rng;

// for error handling, define a struct called Guess. Value will be the users guess, of i32
pub struct Guess {
    value: i32,
}

// on the struct guess give it two methods. a new and a value method.
// new method takes in a value as i32 and returns type Guess. if the value is out of bounds, panic and display the error
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {
            value
        }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }

        Guess::new(guess);

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}
