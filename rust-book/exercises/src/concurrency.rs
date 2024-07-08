use std::{sync::mpsc::{self}, thread, time::Duration};


pub fn exercise() {
    spawn_thread_ex(false);
    println!("");
    spawn_thread_ex(true);
    println!("");
    thread_move_ex();
    println!("");
    message_passing_with_channels();
}

fn spawn_thread_ex(halt_main: bool) {
    let thread_handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i}, from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    if halt_main {
        thread_handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {i} from main thread");
            thread::sleep(Duration::from_millis(1));
        }
    }
    else {
        for i in 1..5 {
            println!("hi number {i} from main thread");
            thread::sleep(Duration::from_millis(1));
        }
        thread_handle.join().unwrap();
    }
}

fn thread_move_ex() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("The vector: {v:?}");
    });

    handle.join().unwrap();
}

fn message_passing_with_channels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}