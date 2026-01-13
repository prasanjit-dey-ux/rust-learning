fn main() {
    println!("Hello, world!");

    another_function(23); // fn calling & the value we are passing is the argument
    distance(6, "km");
}

fn another_function(x: i32) { // this is the parameter
    println!("The value of x is {x}")
}

fn distance(value: i32, unit_label: &str) {
    println!("The distance is {value}{unit_label}")
}

