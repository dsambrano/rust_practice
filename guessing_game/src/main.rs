extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to, Guess the Number Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    // Not taking output and storing it in variable, but rather asking where in
    // memory would you like this stored, e.g., we can a binding for it to use.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You WIN!"),
    }

}
