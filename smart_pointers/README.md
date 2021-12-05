## Summary
This chapter covered how to use smart pointers to make different guarantees and trade-offs from those Rust makes by default with regular references. 

The ```Box<T>``` type has a known size and points to data allocated on the heap. 

The ```Rc<T>``` type keeps track of the number of references to data on the heap so that data can have multiple owners. 

The ```RefCell<T>``` type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type; it also enforces the borrowing rules at runtime instead of at compile time.

Also discussed were the ```Deref``` and ```Drop``` traits, which enable a lot of the functionality of smart pointers. We explored reference cycles that can cause memory leaks and how to prevent them using ```Weak<T>```.