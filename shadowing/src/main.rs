fn main() {
    let x = 2;
    let x = x + 4;

    {
        let x = x * 2;
        println!("the value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "          ";
    let spaces = spaces.len();

    println!("the number of space given are: {spaces}");
}


/*
   - Shadowing means creating a new variable binding with the same name that replaces the previous one in that scope.

   - Shadowing vs Mutability: 
   Its not mutabile like here we can use any type, where as mutability we can only similar type

*/

