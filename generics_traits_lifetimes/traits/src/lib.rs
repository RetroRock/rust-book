use std::fmt::Debug;
use std::fmt::Display;

// Docs: https://doc.rust-lang.org/stable/book/ch10-02-traits.html
// Like interfaces in other languages
// Crates do not share traits

// pub trait Summary {
//     fn summarize(&self) -> String;
// }

// or (default return value)
// pub trait Summary {
//     fn summarize(&self) -> String {
//         // Default value that can be overwritten
//         String::from("(Read more...)")
//     }
// }

// or default implementation where other methods in the same trait,
// even those without a default implementation are, are called
// To use this version of Summary,
//we only need to define summarize_author when we implement the trait on a type
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementation of Summary trait on NewsArticle
// Uses default summary value -> summarize can still be called
// impl Summary for NewsArticle {}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementation of Summary trait on Tweet
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameters -> item is any type that implements the Summary trait
// can call any function in Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax, previous version is just syntax sugar
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2_params(item1: &impl Summary, item2: &impl Summary) {}
// Equivalent using trait bound syntax
pub fn notify_2_params_v2<T: Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the + syntax
pub fn notify_mult(item1: &(impl Summary + Display)) {}
// or using Trait Bound syntax
pub fn notify_mult_v2<T: Summary + Display>(item1: &T) {}
// ! With the two trait bounds specified,
// ! the body of notify can call summarize and use {} to format item.

// Clearer Trait Bounds with were Clauses
// one way to write
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

// less cluttered variant
fn some_function_v2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// Returning types that implement Traits
// In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t know that.
// The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators,
// which we cover in Chapter 13.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// You can only return a single type due to compiler implementation
// won't compile -> working version in “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17
// fn returns_summarizable_v2(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

struct Pair<T> {
    x: T,
    y: T,
}

// Conditional types (only new)
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// Implements cmp_display if inner types implements PartitialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Standard library implements ToString on any type that implements the ToString type
// that's why let s = 3.to_string(); works

// impl<T: Display> ToString for T {
// --snip--
// }
