Summary
// Structs let you create custom types that are meaningful for your domain. 
// By using structs, you can keep associated pieces of data connected to each other 
// and name each piece to make your code clear. 
// Methods let you specify the behavior that instances of your structs have, 
// and associated functions let you namespace functionality that is particular 
// to your struct without having an instance available.


#[derive(Debug)]

struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 { // methods can take ownership of self, borrow self immutably or borrow self mutably
                            // just as any other parameter (self, &mut self, &self)
		self.width * self.height
	}

    fn can_hold(&self, rect: &Rectangle) -> bool {
       self.width >= rect.width &&
           self.height >= rect.height
    }

    fn square(size: u32) -> Rectangle { // this is a associated function, not a method, because self isn't involved
        Rectangle {                     // like String::from()
            width: size,
            height: size,
        }
    }
}

// It's also possible to use mutliple impl blocks, can be useful for generic types and traits ->
// in chapter 7

fn main() {
	let rect1 = Rectangle {
		width: 30,	
		height: 50,
	};	

	println!(
		"The height of the rectangel is {} square pixels.",
		rect1.area()
	);

    let rect2 = Rectangle { 
        width: 10,
        height: 40,
    };
    
    let rect3= Rectangle { 
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3); // using the associated function, :: is also used for namespaces -> chapter 7
}
