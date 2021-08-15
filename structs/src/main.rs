struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// struct BadUser { 
//    username: &str, // does not work because no lifetime specified
//    email: &str,    // references have no ownership -> may not be valid for as long as struct
//    sign_in_count: u64,
//    active: bool,
// }

// Using tuple structs without named fiels to create types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // only works if user1 is mutable (with mut keyword)
    user1.email = String::from("anotheremail@example.com"); 
    
    let user2 = User {
        email: String::from("another@example.com");
        username: String::from("anotherusername567");
        ..user1 // struct udpate syntax 
    }; 
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
//    let user3 = User {
//       email: "someone@example.com", // does not work without lifetimes (chap 10)
//       username: "someusername123",  // because user3 has no ownership of the value
//       active: true,
//       sing_in_count: 1,
//    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
