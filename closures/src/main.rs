// Docs: https://doc.rust-lang.org/stable/book/ch13-01-closures.html
// Closures are anonymouse functions that can be saved in a variable or
// passed as arguments to other functions. Unlike functions, closures can capture values
// from the scope in which they're defined

use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// Example: App that generates custom excersice plans in the backend written in Rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // ! Capturing the environment with closures
    let x = 4;
    // Access variable x out of scope equal_to_x was defined in
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));

    // Attempting to do the same with functions won't compile
    // Functions are not allowed to capture their environment to avoid overhead
    // let x = 4;
    //
    // fn equal_to_x(z: i32) -> bool {
    // z == x // can't capture dynamic variable in a function
    // }
    //
    // let y = 4;
    //
    // assert!(equal_to_x(y))

    /*
    ! FnOnce consumes the variables it captures from its enclosing scope,
    known as the closure’s environment. To consume the captured variables,
    the closure must take ownership of these variables and move them into the closure when it is defined.
    The Once part of the name represents the fact that the closure can’t take ownership
    of the same variables more than once, so it can be called only once.
    ! FnMut can change the environment because it mutably borrows values.
    ! Fn borrows values from the environment immutably.
    */

    // let x = vec![1, 2, 3];

    // ! x value moved due to use in closure
    // ! Integers would get copied instead of moved because Vec does not implement the 'Copy' trait
    // let equal_to_x = move |z| z == x;

    // x moved out of scope
    // println!("can't use x here: {:?}", x);

    // let y = vec![1, 2, 3];

    // assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout_worst(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Today, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn generate_workout_better(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Today, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a brakd today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

fn generate_workout_even_better(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Today, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a brake today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

struct Cacher<T, G>
where
    // Trait bounds on T specify that it's a closure by using the Fn trait.
    // When code using a Cacher asks for the result of the closure, the Cacher will execute the closure
    // at that time and store the result within a Some variant in the value field.
    // Then if the code asks for the result of the closure again,
    // instead of executing the closure again, the Cacher will return the result held in the Some variant.
    G: Hash + Eq + Clone,
    T: Fn(G) -> G,
{
    calculation: T, // will be None at first
    value: HashMap<G, G>,
}

impl<T, G> Cacher<T, G>
where
    G: Hash + Eq + Clone,
    T: Fn(G) -> G,
{
    fn new(calculation: T) -> Cacher<T, G> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: G) -> G {
        let x = self.value.get(&arg);
        match x {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg.clone());
                self.value.insert(arg, v.clone());
                v
            }
        }
        // match self.value {
        // Some(v) => v,
        // None => {
        // let v = (self.calculation)(arg);
        // self.value = Some(v);
        // v
        // }
        // }
    }
}

// ! Best way
fn generate_workout(intensity: u32, random_number: u32) {
    // Call the value method as many times as we want, or not call it at all,
    // the expensive calculation will be run a maximum of once.
    let mut expensive_closure = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Today, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a brakd today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

// ! Closures compared to functions
// Can add types, but usually the compiler figures that out by itself because of the limited context
// Calling the closures is required for add_one_v3 and add_one_v4 to be able to compile
// because the types will be inferred from their usage.
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;

// ! Won't compile (two different types)
// let example_closure = |x| x;
// let s = example_closure(String::from("hello"));
// let n = example_closure(5);

#[cfg(test)]
mod tests {
    use super::*;

    // ! Limitations of the Cacher Implementation
    // Test will fail, because value 1 gets cached and won't be updated afterwards.
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
