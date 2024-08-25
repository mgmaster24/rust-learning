use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

// ******************* Structures for examples ********************************** //
use crate::pointers::pointers::List::{Cons, Nil};
use crate::pointers::pointers::RcList::{RcCons, RcNil};
use crate::pointers::pointers::RcRefCellList::{RcRefCons, RcRefNil};

// Cons list implementation
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
enum RcList<T> {
    RcCons(T, Rc<RcList<T>>),
    RcNil,
}

#[derive(Debug)]
enum RcRefCellList<T> {
    RcRefCons(Rc<RefCell<T>>, Rc<RcRefCellList<T>>),
    RcRefNil,
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

// ******************* End Structures for examples ********************************** //

pub fn exercise() {
    create_and_traverse_list();

    my_box_examples();

    my_smart_pointer_examples();

    rc_list_examples();

    rc_ref_cell_list_examples();
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn add(lhs: &mut i32) {
    *lhs += 17
}

fn create_and_traverse_list() {
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
}

fn my_box_examples() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let rust = MyBox::new(String::from("Rust"));
    hello(&rust);

    let mut mut_val: MyBox<i32> = MyBox::new(42);
    add(&mut mut_val);
    println!("Mutiple MyBox value: {}", *mut_val);
}

fn my_smart_pointer_examples() {
    let c = MySmartPointer {
        data: String::from("Smart Pointer 1"),
    };

    let _d = MySmartPointer {
        data: String::from("Smart Pointer 2"),
    };

    drop(c);
    println!("Two Smart Pointers created!");
}

fn rc_list_examples() {
    let list_a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&list_a));
    let _list_b = RcCons(3, Rc::clone(&list_a));
    println!("count after creating b = {}", Rc::strong_count(&list_a));
    {
        let _list_c = RcCons(4, Rc::clone(&list_a));
        println!("count after creating c = {}", Rc::strong_count(&list_a));
    }
    println!("count after dropping c = {}", Rc::strong_count(&list_a));
}

fn rc_ref_cell_list_examples() {
    let value = Rc::new(RefCell::new(5));
    let list_a = Rc::new(RcRefCons(Rc::clone(&value), Rc::new(RcRefNil)));
    let list_b = RcRefCons(Rc::new(RefCell::new(3)), Rc::clone(&list_a));
    let list_c = RcRefCons(Rc::new(RefCell::new(4)), Rc::clone(&list_a));

    *value.borrow_mut() += 10;
    println!("list_a after: {list_a:?}");
    println!("list_b after: {list_b:?}");
    println!("list_c after: {list_c:?}");
}
