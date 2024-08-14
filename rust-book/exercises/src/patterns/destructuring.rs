struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move(Point),
    Write(String),
    ChangeColor(Color),
}

pub fn exercise() {
    println!("\t----- Begin Destructuring Structs -----");
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("\t\tOn the x axis at {x}"),
        Point { x: 0, y } => println!("\t\tOn the y axis at {y}"),
        Point { x, y } => {
            println!("\t\tOn neither axis: ({x}, {y})");
        }
    }
    println!("\t----- End Destructuring Structs -----\n");
    println!("\t----- Begin Destructuring Enums -----");
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("\t\tThe Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("\t\tMove in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("\t\tText message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("\t\tChange the color to red {r}, green {g}, and blue {b}")
        }
    }

    println!("\t----- End Destructuring Enums -----\n");

    println!("\t----- Begin Destructuring Nested Struct or Enums -----");
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("\t\tChange color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("\t\tChange color to hue {h}, saturation {s}, value {v}")
        }
        Message2::Move(Point { x, y }) => {
            println!("\t\tMoving to {x}, {y}")
        }
        _ => (),
    }
    println!("\t----- End Destructuring Nested Struct or Enums -----\n");

    println!("\t----- Begin Destructuring Structs and Tuples -----");
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("\t\tFeet: {feet} and inches: {inches} from destructured tuple.");
    println!("\t\tPoint x: {x}, y:{y} from  destructured struct.");
    println!("\t----- End Destructuring Structs and Tuples -----");
}
