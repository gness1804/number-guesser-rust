use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // equivalent of `let guess = ''` in JS

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        // like `.catch(() -> {console.log('Failed to read line')})` in JS

    println!("You guessed: {}", guess);
    // JS equivalent: `console.log(`You guessed: ${guess}`);`
}
