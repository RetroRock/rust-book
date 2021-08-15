fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // change(&s1); // doesn't work

    change2(&mut s1); // you can only have one mutible reference in a scope
                      // -> to prevent data races
                      // data race: two or more pointers access the same data at the same time,
                      // at least one of the pinters is being used to write to the data
                      // There's no mechanism being used to synchronize access to the data
    // let r1 = &mut s1;
    // let r2 = &mut s2;
    
    {
        let r3 = &mut s1; // but in new scope it works
    } // s3 goes out of scope here

    {
        let r1= &s1; // no problem
        let r2 = &s1; // no problem
        let r3 = &mut s1; // BIG PROBLEM
        // We cannot have mutable references while we have immutable references
        
        println!("{}, {}, and {}", r1, r2, r3);
    }

    {
        let mut s = String::from("hello");
        
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point
        // that's why the below works 
        let r3 = &mut s; // no problem
        println!("{}", r3);
    }

    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it dow not hav ownership of what
  // it refers to, nothing happens. Ownership does not have to be returned

// fn change(some_string: &String) {    // does not work, because we have no ownership, it's just a reference
//    some_string.push_str(", world"); // and references are immutable by default
// }

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String { // pointer to a place in memory that has been freed -> won't compiled
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
  // solution is to just return String eg s and ownership is moved out
