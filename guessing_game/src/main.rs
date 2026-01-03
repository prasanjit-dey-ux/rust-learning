// Processing a Guess

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    let guess: String = String::new();

    io::stdin().read_line(guess);
    
}
