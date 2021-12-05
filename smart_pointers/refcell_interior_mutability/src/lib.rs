// Docs: https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects

// A Use Case for Interior Mutability: Mock Objects
// & and &mut translates to .borrow() and .borrow_mut() for RefCell<T>
// .borrow() returns the smart pointer type Ref<T>,
// .borrow_mut() returns the smart pointer type RefMut<T>
// Both implement Deref trait -> use them like regular references
// RefCell keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active
// each .borrows() increases that number by one, if Ref<T> goes out of scope that value decreases by one
// Just like compile-time borrowing rules, RefCell<T> allows either many immutable
// borrows ore one mutable borrow at any point in time
// If those are violated, rather than getting a compile time error, the implementation
// will panic at runtime

// RefCell<T> makes it possible to write a mock object that can modify itself
// to keep track of the messages it has seen while you’re using it
// in a context where only immutable values are allowed.
// You can use RefCell<T> despite its trade-offs to get more functionality
// than regular references provide.

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90& of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        // sent_messages: Vec<String>,
        // RefCell for the win
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                // RefCell for the win
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Not a mutable reference (Changing to &mut self would not match Messenger trait definition)
            // self.sent_messages.push(String::from(message));
            // RefCell for the win
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1)
        // RefCell for the win
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
