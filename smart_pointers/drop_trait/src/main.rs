// Docs: https://doc.rust-lang.org/stable/book/ch15-03-drop.html

// Drop lets you customize what happens when a value is about to go out of scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // Free manually: std::mem::drop
             // When c and d go out of scope, drop will be called
             // Variables are dropped in the reverse order of their creation, so d was dropped before c
             // Cannot call drop directly, it's a destructor -> use std::mem::drop
}
