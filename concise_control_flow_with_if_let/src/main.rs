// Docs: https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

#[derive(Debug)]

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
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    } // this a lot of effort for only one case

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value { // works the same as match, but more consice -> syntax sugar
        println!("three");           // trade-off is loosing exhaustive checking that comes with match
    }

    let coin = Coin::Penny;

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    
    let coin = Coin::Penny;

    // Or use if let and else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // USE if let IN CASES WHERE match WOULD BE TO VERBOSE
}
