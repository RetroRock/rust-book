use add_one;
use add_two;

// Use cargo run -p adder to run adder binary from top-level add directory

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );

    println!(
        "Hello, world! {} plus two is {}",
        num,
        add_two::add_two(num)
    );
}
