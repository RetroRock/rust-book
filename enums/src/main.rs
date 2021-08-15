// Docs: https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String), // attach data to the enum directly -> don't have to define a struct
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8), 
    V6(String),
}

// Ip Adress types come with std library
struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IpAddrStd { // you can put any kind of data inside an enum, even another enum
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message { 
    Quit, 
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// struct equivalent
// struct QuitMessage; // unit struct
// struct MoveMessage {
//    x: i32,
//    y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message { // we can also define function on enums
    fn call(&self) {
        // method body would be defined here
    }
}

// enum Option<T> { // comes with std library in the prelude (no need to import it) 
//    Some(T),      // <T> is a generic and means that Some can hold a value of any type
//    None,         // can be used without the `Option::` prefix
// }

fn main() {
   let four = IpAddrKind::V4; 
   let six = IpAddrKind::V6; 

   route(IpAddrKind::V4);
   route(IpAddrKind::V6);

   let home = IpAddr::V4(String::from("127:0.0.1"));
   let loopback = IpAddr::V6(String::from("::1"));

   let home = IpAddr2::V4(127, 0, 0, 1);
   let loopback = IpAddr2::V6(String::from("::1"));

   let m = Message::Write(String::from("hello"));
   m.call();

   let some_number = Some(5);
   let some_string = Some("a string");

   let absent_number: Option<i32> = None; // Rust needs to know type

   let x: i8 = 5;
   let y: Option<i8> = Some(5);

   // let sum = x + y; // this won't compile because you cannot add i8 to Option<i8> -> better than null (would probly compile)
   // we first have to check if the value is not null (or None) -> the compiler will ensure that we
   // deal with this case, generally Option<T> first has to be converted to T to work with it
}

fn route(ip_kind: IpAddrKind) {
    
}
