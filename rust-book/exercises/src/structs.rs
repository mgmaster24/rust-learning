struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rhs: &Rectangle) -> bool {
        self.width > rhs.width && self.height > rhs.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn exercise() {
    println!("---------------Running Structs Exercise---------------");
    let user = User {
        active: true,
        username: String::from("mgmaster"),
        email: String::from("mgmaster@rust-learning.com"),
        sign_in_count: 0,
    };

    println!("User: {}, {}", user.username, user.email);

    let mut user2: User = User {
        active: true,
        username: String::from("mgmaster"),
        email: String::from("mgmaster@rust-learning.com"),
        sign_in_count: 1,
    };
    user2.email = String::from("user2@gmail.com");
    user2.sign_in_count = 2;

    println!("User 2: {}, {}", user.username, user2.email);

    let mut user3 = build_user(String::from("user3@rust.org"), String::from("rusty"));
    user3.active = false;

    println!("User 3: {}, {}", user.username, user3.email);

    let user4 = User {
        email: String::from("rusty@rust.org"),
        ..user3
    };

    println!("User 4: {}", user4.email);

    let red = Color(255, 0, 0);
    println!("Color Red: {}, {}, {}", red.0, red.1, red.2);

    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect = Rectangle { width, height };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("rect is {:?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(42);
    println!("My square: {:?}", square);
    println!("---------------Exiting Structs Exercise---------------\n");
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
