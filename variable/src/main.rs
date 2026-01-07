fn main() {
    // using mut to make it mutable
    let mut age = 22;
  
    println!("Age : {age}");

    age = 23;
    println!("The current age is {age}");

    const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
    
    println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

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

/*
    - let keyword is used to bind the value

    - const keyword is for constant, now in `let` also we cant change, like we need mut for it to change it, 
    - but note: in const we cant use mut so its a constant only which we cant change

    and in const the variable name should be in uppercase and underscore between the words.

    const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;

    - In const we need to give its type, our compiler will not detect it if we dont give.

    - const can be declared in any scope, including the global scope, which makes them useful for values that many part of the code need to know about, where as we can use let outside the scope.
   
    - The constant value should be contant only, like for eg we cant add another non-constant value to it, we can also say from before we should know the contant value.

*/


/* Shadowing:
    - let by using mut we can change the variable but we can change it type
    but through shadowing we can change its type to

*/