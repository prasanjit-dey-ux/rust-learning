fn main() {
    println!("Hello, world!");

    another_function(23); // fn calling & the value we are passing is the argument
    distance(6, "km");
    addition(24, 6);

    // if we add ; then its a statement if not then a expression, lets see

    let y = {
        let x = 4;
        x + 1
    };

    println!("The value of y is: {y}");
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
