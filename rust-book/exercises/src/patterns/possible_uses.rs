pub fn exercise() {
    println!("\t----- Begin Match Example -----");
    let x = Some(1);
    let two = plus_one(x);
    println!("\t\tAdding one to Option using plus_one match method");
    assert_eq!(2, two.unwrap());
    println!("\t\tAssertion that 2 equals 2 passed using plus_one match method");
    println!("\t----- End Match Example -----\n");
    favorite_color(None, "43", false);
    while_let();
    for_pattern()
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn favorite_color(fc: Option<&str>, a: &str, is_tues: bool) {
    println!("\t----- Begin If Let Example -----");
    let fav_color: Option<&str> = fc;
    let is_tuesday = is_tues;
    let age: Result<u8, _> = a.parse();

    if let Some(color) = fav_color {
        println!("\t\tUsing your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("\t\tTuesday is a green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("\t\tUsing purple as the background color");
        } else {
            println!("\t\tUsing orange as the background color");
        }
    } else {
        println!("\t\tUsing blue as the background color");
    }

    println!("\t----- End If Let Example -----\n");
}

fn while_let() {
    println!("\t----- Begin While Let Example -----");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("\t\t{top}")
    }

    println!("\t----- End While Let Example -----\n");
}

fn for_pattern() {
    println!("\t----- Begin For Example -----");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("\t\t{value} is at the index {index}")
    }
    println!("\t----- End For Example -----");
}
