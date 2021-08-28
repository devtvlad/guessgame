use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number from 1 to 9!");
    let secret_number = rand::thread_rng().gen_range(1, 10);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error!");
        println!("You guessed: {}", guess);
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number form 1 to 9!");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big."),
        }
    }
}