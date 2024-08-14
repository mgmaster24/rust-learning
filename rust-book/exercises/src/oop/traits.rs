pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub place_holder: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw the button
        println!("Drawing a Button");
    }
}

impl Draw for TextField {
    fn draw(&self) {
        println!("Drawing TextField");
    }
}

pub fn exercise() {
    let components: Vec<Box<dyn Draw>> = vec![
        Box::new(Button {
            width: 50,
            height: 50,
            label: String::from("My button"),
        }),
        Box::new(TextField {
            width: 50,
            height: 50,
            label: String::from("My TextField"),
            place_holder: String::from("Put something here"),
        }),
    ];

    let screen = Screen { components };
    screen.run()
}
