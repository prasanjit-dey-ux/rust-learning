fn main() {
    let mut num = 0;

    let result = loop{
        println!("Value of number is {num}");

        if num == 10 {
            break num * 2 ;
        }

        num = num + 1;

    };

    println!("This is the {result}");

    for x in 1..=10 {
        println!("the x is {x}");
    }

}


/*  Let now make the num double, then what will be its value

    - it will go to infinite loop as the value will overflow

*/