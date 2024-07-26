//use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess = String::new();
    let secret_number = 15;

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => {
            let diff: i32 = secret_number - guess;
            println!("Too small, hint: add {diff}")
        }
        Ordering::Greater => {
            let diff: i32 = guess - secret_number;
            println!("Too big, hint: subtract {diff}")
        }
        Ordering::Equal => println!("You win"),
    }
}
