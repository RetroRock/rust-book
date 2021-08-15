fn main() {
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("condition was false");
    }
    
    // This does not work because condition results in integer
    // if number {
    //    println!("The number is three");
    // }
    
    if number != 0 {
        println!("number was something other than zero");
    }
    
    // Multiple conditions
    if number % 4 == 0 {
        println!("number is dividable by 4");
    } else if number % 3 == 0 {
        println!("number is dividable by 3");
    } else {
        println!("number is not dividable by 4 or e");
    }

    // Usin if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // type mismatch -> won't work
    // let number = if condition { 5 } else { "six" }; 
    println!("The value of number is: {}", number);

}

