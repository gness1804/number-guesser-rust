extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();
    // equivalent of `let guess = ''` in JS

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        // like `.catch(() -> {console.log('Failed to read line')})` in JS

    println!("You guessed: {}", guess);
    // JS equivalent: `console.log(`You guessed: ${guess}`);`

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!");
        Ordering::Greater => println!("Too big!");
        Ordering::Equal => println!("You win!");
    }
    // sort of like a JS switch statement, I think
}
