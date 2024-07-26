//use rand::Rng;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let diff: i32 = secret_number - guess;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small, hint: add {diff}"),
            Ordering::Greater => println!("Too big, hint: subtract {}", diff.abs()),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
