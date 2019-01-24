pub trait Draw {
    fn draw(&self);
}

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

pub struct Button {}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw a Button!");
    }
}

pub struct SelectBox {}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw a SelectBox!");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button{}),
            Box::new(SelectBox{}),
        ],
    };

    screen.run();
}
