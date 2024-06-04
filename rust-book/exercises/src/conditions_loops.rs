pub fn exercise() {
    println!("---------------Running Loops And Conditions Exercise---------------");

    // conditionals and loops
    let val = add(24, 19);
    println!("Value of add: {val}");
    let resp = if val == 42 {
        "You've found the meaning of life."
    } else {
        "Just another number."
    };
    println!("{resp}");
    let mut x = 0;
    for number in 1..1000 {
        x += number;
        if x >= 1000 {
            break;
        }
    }

    println!("---------------Exiting Loops And Conditions Exercise---------------\n");
}

fn add(x: i64, y: i64) -> i64 {
    let val = x + y;
    val
}
