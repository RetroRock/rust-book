fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    // println!("{}",guess);

    // INTEGER TYPES
    // unsigned means only positive, signed means also negative, but range goes to 50/50
    let binary = 0b1111_0000;
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let byte = b'8'; // u8 only
    // there is alos isize and usize -> for indexing collections or so, try to fit the system,
    // maybe for sets
    // println!("INTEGER TYPES -> Binary {}, Decimal {}, Hexadecimal {}, Octal {}, Byte {}", 
             // binary, decimal, hex, octal, byte); 
    
    // FLOATING POINT NUMBERS
    let x = 2.0; // f64 -> more precise and as fast

    let y: f32 = 3.0; // f32

    // println!("FLOATING-POINT TYPES -> f64 {}, f32 {}", x, y);
    
    // NUMERIC OPERATIONS
    // addition
    let sum = 5 + 10;
    
    // subtraction
    let subtraction = 95.5 - 4.3;
    
    // multiplication
    let multiplication = 4 * 30;
    
    // division
    let qoutient = 56.7 / 32.3;

    // remainder (modulo)
    let remainder = 43 % 5;

    // println!("NUMERIC OPERATIONS -> {}, {}, {}, {}, {}", 
             // sum, subtraction, multiplication, qoutient, remainder);
    
    // BOOLEAN TYPE (Are one byte in size)
    let t = true;
    let f: bool = false; // with expclit type annotation
   
    // println!("BOOLEAN TYPE -> {}, {}", t, f);
    
    // CHARACTER TYPE (four bytes in size)
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
   
    // println!("CHARACTER TYPE -> {}, {}, {}", c, z, heart_eyed_cat);
    
    // COMPOUND TYPES (Group multiple values into one type)
    // Tuple has fixed size, cannot grow or shrink
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    
    // ARRAY TYPE (every element must be the same type; good when data should go on stack; 
    // cannot grow or shrink)
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array with custom type
    let a = [3; 5]; // array with 5 times number 3

    let first = a[0];
    let second = a[1];

    
}

    
    
