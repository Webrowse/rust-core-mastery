// Vec need items of same size, but we want to store different shapes
// Trait objects allow for dynamic dispatch and heterogeneous collections

trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button with label: {}", self.label);
    }
}

struct TextField {
    serial_no: u32,
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing a text field with placeholder: {}", self.serial_no);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for item in &self.components {
            item.draw();
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                label: String::from("Submit"),
            }),
            Box::new(TextField {
                serial_no: 4,
            }),
        ],
    };
    
    screen.run();
}