#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect_a = Rectangle {
        width: 2,
        height: 5,
    };

    let rect_b = Rectangle {
        height: 6,
        ..rect_a
    };

    let rect_c = Rectangle {
        width: 1,
        height: 4,
    };

    println!("Rectangle A: {:#?}", rect_a);
    println!("Area of rectangle A is: {}", rect_a.area());
    println!("Can rectangle A hold B? {}", rect_a.can_hold(&rect_b));
    println!("Can rectangle A hold C? {}", rect_a.can_hold(&rect_c));
}
