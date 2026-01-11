fn main() {
    /*
       Compound types group multiple values into one values.

       Rust has two primitive Compound types:
       - Tuple
       - Array
    */

    // Tuple
    let user = ("Alice", 25, true);
    // println!("{:?}",user); this is a debug way the better way is destructuring and printing fields

    // Accessing tuple values
    let (name, age, active) = user;

    println!("Name: {name}, Age: {age}, Active: {active}");

    // index access
    let name = user.0;
    let age = user.1;

    println!("{name} & {age}");

    /*
      // The unit type

       let x = ();

       Means nothing
       used when a function return nothing


       fn log() {
           println!("hello");
       }
       it return nothing

       so in simple term tuple is a small fixed packet of data
    */

    /*
       - Can store different values
       - fixed length
       - Treated as one value

       When to use Tuple
       - Grouping related but different values
       - returning multiple values from a function
    */


// Array

    /* In array all the value need to be of same type and it need to be fixed length
        Array isnt flexible as vector types.

        it is most usefull when we know the number of element
    */
    let numbers:[i32; 4] = [1, 2, 3, 4];
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    
    println!("zeros is {:?}", zeros);
    println!("Number is {}", numbers[2]);
    // Array index out of bound
    // println!("Number is {}", numbers[5]);  it will pannic

    /*
        it shows us how much rust memory is save
    */

}
