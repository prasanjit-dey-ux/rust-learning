// Processing a Guess

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed {}", guess);
}

/* So there are two types of Creates
    - Binary Crates 
    - Library Crates

    Binary crates are executable on its own, whereas library crates are not, its intended to be used in other programs.

    rand is a library crates to generate random number.
*/