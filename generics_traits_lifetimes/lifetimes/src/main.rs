// Docs: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html

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
