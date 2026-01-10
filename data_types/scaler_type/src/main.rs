/*
    Rust is a statically typed language, which means that it must know the types of all variables at compile time.   
*/

fn main() {

    // Rust is staticaly type language - like the compiler must know type of every variable at compile time.

    // does it mean we have to write type every time no

    let x = 5; // rust know this is i32

    
    // compiler cannot infer
    let guess : u32 = "42".parse().unwrap();

    println!("{guess}");
    /*
        why?

        "42".parse() can become i32, u32, i64, etc
        - Rust refuse to guess, to not cause problem.
        - Rust is strict because guessing types causes bugs.
     */

    /*
        Integer
        : Signed - i - can be negative
        : Unsigned - u - can not be negative

        why we have so many sizes in rust becuase memory + performance matter.
        
        u8 is used for bytes, raw data
        i32 is for default and general use cases
        usize is for indexing arrays, vectiors

        Rules to follow: 
        Use i32 unless you have a reason not to.
        Use usize for indexing arrays, vectiors.

     */

    let a: i32 = -30;
    let b: u32 = 10;

    println!("signed integer :{a}/n unsigned integer{b}");

    let int_num = 10;
    let float_num = 3.16;
    let is_active = true;
    let letter = "P";
  
    println!("{int_num}, {float_num}, {is_active}, {letter}");

// Integer literal like how rust read numbers

let a = 1_000; // readability;
let b = 0xfff; // hex
let c = 0b1010; // binary
let d = b'A'; // byete (u8)

// Rust doesn’t care how you write it — type matters, not format.


/*

*/
}
