// Processing a Guess

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break
            },
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("To Small!"),
        };
    }
}

/* So there are two types of Creates
    - Binary Crates
    - Library Crates

    Binary crates are executable on its own, whereas library crates are not, its intended to be used in other programs.

    rand is a library crates to generate random number.

    // enter
    - so before when we enter the number in guess_number the enter also use to enter it was like 20/n that why we needed trim in the guess

    // shadowing
    we can use guess again due to shadowing becuase it allow it and remove the previous the guess

    match enforses to check every possible scenario
*/
