extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    //added the `: u32` that isn't in the tutorial in order to practice annotating type

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();
        // equivalent of `let guess = ''` in JS
        // as with above example, I added the type annotation that isn't in the tutorial in order to practice it

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            // like `.catch(() -> {console.log('Failed to read line')})` in JS

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oops! You need to enter a number!");
                continue;
            },
        };
    // added error handling not in the tutorial to tell the user to enter a number

        println!("You guessed: {}", guess);
        // JS equivalent: `console.log(`You guessed: ${guess}`);`

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        // sort of like a JS switch statement, I think
    }
}
