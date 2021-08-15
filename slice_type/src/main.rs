// SUMMARY
// The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. 
// The Rust language gives you control over your memory usage in the same way as other systems programming languages,
// but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have 
// to write and debug extra code to get this control.

fn main() {
   let mut s = String::from("hello world");

   let word = first_word(&s); // word will get the value 5 
   
   s.clear(); // this empties the String, making it equal to ""

   // word still has the value 6 here, but theres no more string that 
   // we could meaningfully use the value 6 with. word is now totally invalid!
   
   { // String slices
    let s = String::from("hello world");
    // this know takes a reference to only a portion of the String
    let hello = &s[0..5]; // &s[..5] is the same
    let world = &s[6..11]; // &s[6..] is the same
    // entire string would be &s[..]
    // Important: only works with one byte characters -> ASCII
    // Multibyte characers in chapter 8
    
    let word = first_word_slice(&s);
    println!("First word: {}", word);
   }

   {
    let mut s = String::from("hello world");

    let word = first_word_slice(&s); // but no error with first_word

    // s.clear(); // error!

    println!("the first word is: {}", word);
   }

   {
    let my_string = String::from("hello world");

    // first_word_slice works on slices of `String`s 
    let word = first_word_slice(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word_slice works on slcies of string literals
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals are string literals already,
    // this works too, without the slice syntax!
   }

   // OTHER SLICES
   let a = [1, 2, 3, 4, 5];

   let slice = &a[1..3]; // has the type &[i32]

   assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
} 
fn first_word_slice(s: &str) -> &str { // works with both String and str str is string slice
// already
// fn first_word_slice(s: &String) -> &str { // better because compiler will ensure 
                                          // that the references to the String remain valid
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
