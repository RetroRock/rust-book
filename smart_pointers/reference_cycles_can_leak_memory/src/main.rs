// Reference Cycles Can leak Memory
// Docs: https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html

// We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>:
// it’s possible to create references where items refer to each other in a cycle.
// This creates memory leaks because the reference count of each item in the cycle will never reach 0,
// and the values will never be dropped.

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    // println!("a initial rc count = {}", Rc::strong_count(&a));
    // println!("a next item = {:?}", a.tail());

    // let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    // println!("a rc count after b creation = {}", Rc::strong_count(&a));
    // println!("b initial rc count = {}", Rc::strong_count(&b));
    // println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    // println!("b rc count after changing a = {}", Rc::strong_count(&b));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
    // println!("Hello, world!");
    // The memory allocated to the list will remain uncollected forever,
    // (because both lists in memory reference each other)
    // Visualisation: https://doc.rust-lang.org/stable/book/img/trpl15-04.svg

    // Creating a Tree Data Structure: a Node with Child Nodes

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Once we have the Node instance in branch,
        // we can modify leaf to give it a Weak<Node> reference to its parent.
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
// Strong references are how you can share ownership of an Rc<T> instance.
// Weak references don’t express an ownership relationship.
// They won’t cause a reference cycle because any cycle involving some weak references will be broken
// once the strong reference count of values involved is 0.

// Creating a Tree Data Structure: a Node with Child Nodes
// We want a Node to own its children, and we want to share that ownership with variables
// so we can access each Node in the tree directly. To do this, we define the Vec<T> items
// to be values of type Rc<Node>. We also want to modify which nodes are children of another node,
// so we have a RefCell<T> in children around the Vec<Rc<Node>>.

// A parent node should own its children, if it is dropped, its children should be dropped as well.
// A child should not own its parent (reference cycle), if the child node is dropped, the parent node should
// still exists -> use case for weak references

// Docs: https://doc.rust-lang.org/stable/book/ch15-06-reference-cycles.html#adding-a-reference-from-a-child-to-its-parent
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
