// constants are always immutable, cannot be changed with mut
const THIS_IS_A_CONSTANT: u32 = 100_000;

fn main() {
    println!("Here you have a constant: {}", THIS_IS_A_CONSTANT);
    let mut x = 5;
    println!("The valouf ox is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // Shadowing
    let y = 5;
    // still immutable
    let y = y + 1;
    // still immutable
    let y = y * 2;

    // Type of variable can also be changed, but the same name can still be used
    let spaces = "   ";
    let spaces = spaces.len();

    // This doesn't work with mut
    let mut spaces = "   ";
    // spaces = spaces.len(); // This won't compile

    println!("The value of y is: {}", y);
}
