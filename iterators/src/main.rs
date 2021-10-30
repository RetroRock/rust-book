fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got {}", val);
    }
}

/*
! The Iterator trait and the next Method
The Iterator trait only requires implementors to define one method:
the next method, which returns one item of the iterator at a time wrapped in Some and,
hen iteration is over, returns None.
*/
// pub trait Iterator {
// type Item; // associated type -> Chapter 19
// fn next(&mut self) -> Option<Self::Item>;
// methods with default implementations elided
// }

// PartialEq -> part of struct can be equal?
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// ! Creating Iterators with the Iterator trait
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // More in Chapter 19
    // associated Item type for iterator -> Iterator will return u32 values
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // mut because internal state is modified
        let mut v1_iter = v1.iter();
        // .iter -> Iterator over immutable references
        // .into_iter -> takes ownership of v1 and returns owned values
        // .iter_mut -> Iterator over mutable references

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // ! Methods that Produce Other Iterators
        // Other methdos defined on the Iterator trait, known as iterator adaptors,
        // allow changing iterators into different kinds of iterators (for example map)
        let v1 = vec![1, 2, 3];
        // v1.iter().map(|x| x + 1); // Warning: iterators are lazy and do nothing unless consumed
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    /*
    Methods that call next are called consuming adaptors, because calling them uses up the iterator
    For example sum -> takes ownership of the iterator and interates through the items by repeatedly calling next
    -> consuming it
    */
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        // Cannot use v1_iter after the call to sum because sum takes ownership of the iterator it gets called on

        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        // Compiler provides default implementations for all these methods, because Iterator trait is implemented
        // zip returns None when either of its input iterators return None
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
