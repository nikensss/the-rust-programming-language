pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing a button: {:?}", &self);
    }
}

// using generics instead of trait objects would limit our screen to have components of only
// Button or only SelectBox; but with trait objects we can have a vector that holds references to
// any type that implements the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
