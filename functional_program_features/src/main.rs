// Docs: https://doc.rust-lang.org/stable/book/ch13-01-closures.html
// Closures are anonymouse functions that can be saved in a variable or
// passed as arguments to other functions. Unlike functions, closures can capture values
// from the scope in which they're defined

use std::thread;
use std::time::Duration;

// Example: App that generates custom excersice plans in the backend written in Rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
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

struct Cacher<T>
where
    // Trait bounds on T specify that it's a closure by using the Fn trait.
    // When code using a Cacher asks for the result of the closure, the Cacher will execute the closure
    // at that time and store the result within a Some variant in the value field.
    // Then if the code asks for the result of the closure again,
    // instead of executing the closure again, the Cacher will return the result held in the Some variant.
    T: Fn(u32) -> u32,
{
    calculation: T, // will be None at first
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
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
