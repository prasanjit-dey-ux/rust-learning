fn main() {
    println!("Hello, world!");

    another_function(23); // fn calling & the value we are passing is the argument
    distance(6, "km");
    addition(24, 6);

    // if we add ; then its a statement if not then a expression, lets see

    let y = {
        let x = 4; // statement
        x + 1 // exoression
    };
        println!("The value of y is: {y}");
        
    let p = substraction(10, 5); // in rust we dont use named argument when calling function.
    println!("value of p is {p}");

    let q = another_example(5);
    println!("The value of q is {q}");

    let check = is_even(4);
    println!("The check is a {check}");

}

fn another_function(x: i32) { // this is the parameter
    println!("The value of x is {x}")
}

fn distance(value: i32, unit_label: &str) {
    println!("The distance is {value}{unit_label}")
}

fn addition(x: i32, y: i32) {
    let add = x + y;
    println!("The addition is {add}");
}

/* Calling a function is an expression */


fn substraction(x: i32, y: i32) -> i32 {
    x - y
}

fn another_example(x:i32) -> i32 {
    x + 1
}

/* Early return */

fn is_even(x: i32) -> bool {
    
    if x % 2 == 0 {
        return true;
    }
    println!("the number is not even");
    return false; 
}