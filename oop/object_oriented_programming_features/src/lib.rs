// Docs: https://doc.rust-lang.org/stable/book/ch17-01-what-is-oo.html

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Encapsulation that Hides Implementation Details
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.udpate_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.udpate_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn udpate_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Polymorphism: Defining a Trati for Common Behaviour (with trait objects)
pub trait Draw {
    fn draw(&self);
}

// Box<dyn Draw> is a trait object (more like objects in other languages)
// A genreic type parameter can only be substituedd with one concrete type at a time,
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
// (For example for use as an library in)
pub struct Screen {
    // stand-in for any type inside Box that implements the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}



// Same using generics
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
