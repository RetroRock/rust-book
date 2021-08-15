use std::io;

fn main() {
    let mut counter = 1;
    let result = loop {
        counter += 1;    
        
        if counter == 100_000 {
            break counter * 2; // this is being returned
        }
    };

    println!("The result is {}", result);

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping through a collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    // for ... in range (.rev() reverses the range)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
     
    let mut fib_index = String::new();
   
    println!("Please type a fibonacci index");
    io::stdin()
        .read_line(&mut fib_index)
        .expect("Failed to read line");

    let fib_index: u128 = fib_index.trim().parse().expect("Please type a number!");
    println!("{}", fib_index);
    let fib = fibonacci(fib_index);
    println!("fibonacci number is {}", fib);
}

// max is 94 for u64, and obviously 186 for u128
fn fibonacci(nth_number: u128) -> u128 {
    let mut fibonacciLeft: u128 = 1;
    let mut fibonacciRight: u128 = 1;
    let mut fibonacci: u128 = 0;
    
    if(nth_number == 1 || nth_number == 2) {
        return 1;
    }

    for x in (2..nth_number) {
       fibonacci = fibonacciLeft + fibonacciRight;
       fibonacciLeft = fibonacciRight;
       fibonacciRight = fibonacci;
    }
    return fibonacci;
}
