fn main() {
    // using mut to make it mutable
    let mut age = 22;
  
    println!("Age : {age}");

    age = 23;
    println!("The current age is {age}");
}

/*  So in rust the variables are immutable 
    - Rust assumes safty first
    - if value dont change, it reduce bugs

    To make it mutable we use mut
    eg:

    let mut x = 6;
    x = 7;

    So In Rust, nothing changes unless you explicitly allow it
*/