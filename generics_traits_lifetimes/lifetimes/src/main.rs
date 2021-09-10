// Docs: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
use std::fmt::Display;

fn main() {
    //   let r;

    //    {
    // let x = 5;
    //  r = &x;
    // } // r goes out of scope
    // breaks
    // println!("r: {}", r);

    // Generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    // ! Won't compile: By the time we reach the println statement, string2 already went out of scope,
    // ! and the reference becomes invalid -> works, because fn longest specified lifetimes,
    // ! if not specified, the compiler would throw an error in fn longer (description of longer)
    // {
    //     let string1 = String::from("long string is long");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is {}", result)
    // }

    // ! Lifetime Annotations in Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence, // reference -> needs lifetime
    };

    // ! The Static Lifetime
    // Lifes as long as the program
    //The text of this string is stored directly in the program’s binary, which is always available.
    // Therefore, the lifetime of all string literals is 'static.
    let s: &'static str = "I have a static lifetime";
}

// ! Compiler doesn't know where &str is borrowed from (x or y) -> won't compile
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }

// ! Lifetime Annotation Syntax
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// 'a is the smaller lifetime of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    y
}

// ! Thinking in Terms of Lifetimes
// Will also compile, because only x is used
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// Does not compile, because return value lifetime is not related to the lifetime of the parameters
// dangling refernce, because result.as_str() is a reference to result, but result goes out of scope
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

// ! Lifetime Annotations in Struct Definitions
// when structs hold references, they need lifetimes

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// ! Lifetime Elision
// Works, because lifetimes are infered by compiler
// same as: fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.

// 3 lifetime rules, 1 for inputs. 2 and 3 for outputs 3 = &self or &mut self

// ! Lifetime Annotations in Method Definitions
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Example where the third lifetime elision rule applies
// no lifetime annotation for method needed, because it lifes as long as the struct (ImportantExcerpt)
// return type gets lifetime of &self
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ! Generics, Traits and Lifetimes combined
// <'a, T> -> because lifetimes and generics are both some type of generic
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if (x.len() > y.len()) {
        x
    } else {
        y
    }
}
