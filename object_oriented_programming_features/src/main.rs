use object_oriented_programming_features::{Button, Draw, Screen};

fn main() {
    let select_box = SelectBox {
        width: 3,
        height: 4,
        options: vec![
            String::from("Option 1"),
            String::from("Option 2"),
            String::from("Option 3"),
        ],
    };

    let button = Button {
        width: 30,
        height: 20,
        label: String::from("OK"),
    };

    let screen = Screen {
        components: vec![Box::new(select_box), Box::new(button)],
    };

    screen.run();
}

// This concept—of being concerned only with the messages a value responds to
// rather than the value’s concrete type—is similar to the concept of duck typing
// in dynamically typed languages: if it walks like a duck and quacks like a duck,
// then it must be a duck! In the implementation of run on Screen,
// run doesn’t need to know what the concrete type of each component is.
// It doesn’t check whether a component is an instance of a Button or a SelectBox,
// it just calls the draw method on the component.
// By specifying Box<dyn Draw> as the type of the values in the components vector,
// we’ve defined Screen to need values that we can call the draw method on.

// ! Trait Objects Perform Dynamic Dispatch
// (https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)

// static dispatch = compiler knows what method you're calling at compile time and generates
// nongeneric implementations of functions and methods for each concretete type
// that is used in place of generic type parameter (Monomorphization)

// dynamic dispatch = the compiler can't tell at compile time which method is about to be called
// it emits code that at runtime will figre out which method to call (for library use)

// Rules for trait objects:
// 1. The return type isn't Self
// 2. There are no generic ype parameters

// Example:
// pub trait Clone {
//    fn clone(&self) -> Self;
//}
// pub struct Screen {
//     pub components: Vec<Box<dyn Clone>>, // does not work, because CLone trait returns Self
// }

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
