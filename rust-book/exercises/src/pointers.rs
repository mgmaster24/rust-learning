use std::ops::{Deref, DerefMut};

use crate::pointers::List::{Cons, Nil};

// Cons list implementation
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with data `{}`!", self.data);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn add(lhs: &mut i32) {
    *lhs += 17
}

pub fn exercise() {
    let mut cons_list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    loop {
        match cons_list {
            Cons(v, l) => {
                println!("{}", v);
                cons_list = *l;
            }
            Nil => {
                println!("Nil");
                break;
            }
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let rust = MyBox::new(String::from("Rust"));
    hello(&rust);

    let mut mut_val = MyBox::new(42);
    add(&mut mut_val);
    println!("Mutiple MyBox value: {}", *mut_val);

    let c = MySmartPointer {
        data: String::from("Smart Pointer 1"),
    };

    let d = MySmartPointer {
        data: String::from("Smart Pointer 2"),
    };

    drop(c);
    println!("Two Smart Pointers created!");
}
