use chapter_17::{Button, Draw, Screen};

#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing a select box: {:?}", &self);
    }
}

impl SelectBox {
    fn get_width(&self) -> &u32 {
        &self.width
    }

    fn get_height(&self) -> &u32 {
        &self.height
    }

    fn get_options(&self) -> &Vec<String> {
        &self.options
    }
}

fn main() {
    let select_box = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    };

    println!("Select box details:");
    println!("\twidth: {}", select_box.get_width());
    println!("\theight: {}", select_box.get_height());
    println!("\toptions: {:?}", select_box.get_options());

    let screen = Screen {
        components: vec![
            Box::new(select_box),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Ok"),
            }),
        ],
    };

    screen.run();
}
