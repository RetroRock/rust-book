// Docs: https://doc.rust-lang.org/stable/book/ch06-02-match.html
#[derive(Debug)] // so we can inspect the state in a minute

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska)); 

    let five = Some(5); // Some(5) matches Some(i) (i binds to the value contained in Some) 
    let six = plus_one(five);
    let none = plus_one(None); 

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        5 => println!("seven"),
        _ => (), // cover all all remaining values to 255 (do nothing)
    } // if let if we only care about one of the cases -> chapter 18
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // pretty similar to switch in my opinion
        Coin::Penny => {
            println!("Lucky penny");
            1
        }, 
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // getting the inner Sate of Coin::Quarter
            println!("State quarter from {:?}!", state); 
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // matches are exhaustive -> we have to handle the None case (all cases) -> won't compile otherwise
        Some(i) => Some(i + 1), // i is potential value in x, i guess  
    }
}

