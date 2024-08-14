enum Message {
    Hello { id: i32 },
}

pub fn exercise() {
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_var @ 3..=7 } => println!("\tFound an id in range: {id_var}"),
        Message::Hello { id: 10..=12 } => println!("\tFound an id in another range"),
        Message::Hello { id } => println!("\tFound some other id: {id}"),
    }
}
