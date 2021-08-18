// the top module is 'crate'

// use std::cmp::Ordering;
// use std::io;
// or
use std::{cmp::Ordering, io};

// brings std::io and std::io::Write into scope
use std::io::{self, Write};

// brings all items in std::collections into scope -> be careful, you don't know what exactly has
// been imported
use std::collections::*;

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // cook_order();
        super::serve_order(); // in order to access serve_order in parent module super has to be used, it's like ../ or ..
    }

    pub struct Breakfast {
        pub toast: String, // have to explicitly make it public to use
        seasonal_fruit: String, // is still private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { 
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer { // public enums make all their values public
        Soup,
        Salad,
    }
}

mod front_of_house { // defining a module
    pub mod hosting { // defining another module inside of it i, pub to use it from the outside, otherwise always private
                      // contents also have to have pub in order to be used from the outside
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // bring module hosting into scope
// use self::front_of_house::hosting; // relative path alternative to above
// use crate::front_of_house::hosting::add_to_waitlist; // we can also import function itself,
// to avoid having to call hosting::add_to_waitlist() all the time, but the above solution is a
// better practive for functions

// No external code can also use hosting (because of pub)
// pub use create::front_of_house::hosting;

// other items like structs, enums should be brought into scope using the full scope like a so
use std::collections::HashMap; 

// as keyboard for especially for modules that have the same name
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {}
fn function2() -> IoResult<()> {}

// Using modules from seperate files
pub use crate::front_of_house_separate::hosting as hosting_separate;

pub fn eat_at_restaurant() {
    // Absolute path, front_of_house isn't public, but defined in the same context -> siblings can
    // access it
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye"); // needs the function to actually create an instance of Breakfast
    // Change your mind about what brea we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that ocmes with the meal
    // meal.season_fruit = String::from("blueberries");
    
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    hosting_separate::add_to_waitlist();
    hosting_separate::add_to_waitlist();
    hosting_separate::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}
