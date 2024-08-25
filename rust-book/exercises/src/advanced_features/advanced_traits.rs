use std::{
    fmt,
    ops::{Add, Deref, DerefMut},
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("\t\tThis is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("\t\tUp!");
    }
}

impl Human {
    fn fly(&self) {
        println!("\t\t*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn exercise() {
    operator_overloading();
    fully_qualified_syntax();
    using_super_traits();
    new_type_pattern();
}

fn operator_overloading() {
    println!("\t----- Begin Operator Overloading -----");
    println!("\t\tAdding two points using overloaded operation");
    let res = Point { x: 1, y: 0 } + Point { x: 3, y: 2 };
    assert_eq!(res, Point { x: 4, y: 2 });
    println!("\t\tResult: {:?}", res);

    println!("\t\tAdding meters to millimeters using overloaded operation that returns Millimeter");
    let meters: Meters = Meters(3);
    let millis: Millimeters = Millimeters(600);

    let total_millis = millis + meters;
    assert_eq!(total_millis.0, 3600);

    println!("\t\tTotal millimeters: {}", total_millis.0);
    println!("\t----- End Operator Overloading -----");
}

fn fully_qualified_syntax() {
    println!("\t----- Begin Fully Qualified Syntax -----");
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);

    println!("\t\tA baby dog is called a {}", Dog::baby_name());
    println!(
        "\t\tCorrected: A baby dog is called a {}",
        <Dog as Animal>::baby_name()
    );
    println!("\t----- End Fully Qualified Syntax -----");
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("\t\t{}", "*".repeat(len + 4));
        println!("\t\t*{}*", " ".repeat(len + 2));
        println!("\t\t* {output} *");
        println!("\t\t*{}*", " ".repeat(len + 2));
        println!("\t\t{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

fn using_super_traits() {
    println!("\t----- Begin Using Super Traits -----");
    let p = Point { x: 42, y: 69 };
    p.outline_print();
    println!("\t----- End Using Super Traits -----");
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }
}

fn new_type_pattern() {
    println!("\t----- Begin Newtype Pattern - External Traits -----");
    let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("\t\tw = {w}");
    (*w).push(String::from("new value"));
    println!("\t\tw = {w}");
    println!("\t----- End Newtype Pattern - External Traits -----");
}
