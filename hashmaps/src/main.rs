// Are the least used ones compared to strings and vectors,
// therefore you have to import them
// Data is stored on the heap
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Keys of types string and values of type i32
    // All keys must have the same type, same goes for values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip creates new vector of tuples, collect turns that into a hashmap
    // collect does not only do hashmaps
    // Rust infers types from value in <_, _>
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and see what compiler error you get
    // i32 implements copy trait -> get's copied into hashmap, String doesn't -> values will be moved and map will be the owner of those values
    // If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid. We’ll talk more about these issues in the “Validating References with Lifetimes” section in Chapter 10.

    // Accessing Values in a Hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // if no value get will return None (Result is wrapped in Some(&10))
    // get returns an Option<&V>
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    // overwrite an existing value, ignore the new value if key already exists combine both

    // Owerwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Only inserting a Value If the Key Has No Value
    // check with entry -> returns an enum called Entry that represents the value that might or might not exist

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // the or_insert method on Entry is defined to return a mutabble reference to the value for the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.

    println!("{:?}", scores);

    // Updating a value based on t he Old Value

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // This code will print {"world": 2, "hello": 1, "wonderful": 1}.
    // The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
    // Here we store that mutable reference in the count variable, so in order to assign to that value,
    // we must first dereference count using the asterisk (*).
    // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

    println!("{:?}", map);
}
