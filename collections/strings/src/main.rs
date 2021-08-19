// Book: https://doc.rust-lang.org/stable/book/ch08-02-strings.html

fn main() {
   // Creating a new empty string;
   let mut s = String::new();

   // String with initial data
   let data = "initial contents";

   // to_strint() available on all types that implement the Display trait
   let s = data.to_string();

   // the method also works on a literal directly:
   let s = "initial contents".to_string();
    
   // equivalent to .to_string()
   let s = String::from("initial contens");

   // Strings in Rust are UTF-8 encoded
   let hello = String::from("السلام عليكم");
   let hello = String::from("Dobrý den");
   let hello = String::from("Hello");
   let hello = String::from("שָׁלוֹם");
   let hello = String::from("नमस्ते");
   let hello = String::from("こんにちは");
   let hello = String::from("안녕하세요");
   let hello = String::from("你好");
   let hello = String::from("Olá");
   let hello = String::from("Здравствуйте");
   let hello = String::from("Hola");
    
   // Updating a String
   let mut s = String::from("foo");
   // takes a string slice, we don't want ownership
   s.push_str("bar");

   let mut s1 = String::from("foo");
   let s2 = "bar";
   s1.push_str(s2);
   // it would be bad if we couldn't use s2, because push_str takes ownership of s2
   println!("s2 is {}", s2); 

   // Add a single character to a string
   let mut s = String::from("lo");
   s.push('l');
    
   // Concatenation
   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   let s3 = s1 + &s2; // note s1 has been mode here and can no longer be used
   // + sort of like fn add(self, s: &str) -> String { , but in Rust it is implemented using
   // generics (chap 10)  
   // &String to &str (&s2 is &String) works, because Rust does dere coercion, which turns &s2 into
   // &s2[..] (more of deref coercion in chap 15) 

   // Concatenating multiple strings
   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s = s1 + "-" + &s2 + "-" + &s3;

   // Alternative to that:
   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");
    
   // doesn't take ownership of any of its values (how is that possible without & though?)
   let s = format!("{}-{}-{}", s1, s2, s3);

   // Indexing into Strings
   let s1 = String::from("hello");
   // does not work, because of how Rusts stores strings in memory
   // let h = s1[0];
   
   // Internal representation
   // len will be 4, because the string uses 4 bytes
   let hell = String::from("Hola");
   // len will be 24 instead of 12, because that's the number of bytes it needs to store, each
   // scalar value takes up 2 bytes
   let hello = String::from("Здравствуйте");
   // &hello[0] should be the first letter which is 208 in unicode and the second is 151, so it
   // should be 108, but 208 is not a valid character on its own, that's why &hello[0] would not
   // compile
    
   // Bytes and Scaler Values and Grapheme Clusters
   // Rust can think of strings as bytes, scalar values and grapheme clusters (which is the closest
   // thing to what we would call letters)
   
   // The Hindi word "नमस्ते" (something is messed up here) is implemenented using a vector of u8
   // values that looks like thsi
   // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
   // That's 18 bytes and is how computers store this data. If we look at them as Unicde scaler
   // values, which are what Rust's char type is, those bytes look like this:
   // ['न', 'म', 'स', '्', 'त', 'े']
   // There are six char values here, but the fourth and sixth are not letters: they're diacirtics
   // that don't make sense on their own. Finally, if we look at them as grapheme clusters, we'd
   // get what a person would call the four letters that make up the Hindi word:
   // ["न", "म", "स्", "ते"]
   // You can choose how to interpet the raw stirng data that computers store
   // The final reason Rust doesn't allow indexing into a String to get a character is that
   // indexing operations are expected to always take constant time (O(1)). But it's not possible
   // to guarantee that performance with a Stirng, because Rust would have to walk thorugh the
   // contents form the beginning to the index to determine how many valid characters there were.

   // Slicing types
   // Indexing into a string is often a bad idea because it's not clear what the return type of the
   // string-indexin operation should be: a byte value, a character, a grapheme cluster, or a string
   // slice. Therefore Rust asks you to be more specific if your really need to use indices to
   // create string slices. Rather than indexing with a single number, you can use [] with  a range
   // to create a string slice containing particular bytes:
   let hello = "Здравствуйте";
   // first 4 characters of the &str in this case, because s is &str. Each character is 2 bytes.
   let s = &hello[0..4];
   // running &hello[0..1] means, that Rust would panic at runtime in the same way as an invalid
   // index were accessed in a vector: 'bute index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`
   // Use ranges to create string slices with caution, because doing so can crash your program
   
   // Methods for Iterating Over Strings
   for c in "Здравствуйте".chars() {
        println!("{}", c);
   }

   for b in "Здравствуйте".bytes() {
        println!("{}", b);
   }

   // Gettings grapheme clusters from strins is complex, to this functionality is not provided with
   // the standard library. Crates for this are available on crates.io
   
   // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte
}
