pub fn exercise() {
    println!("\t----- Begin Multiple Patterns With Or -----");
    let x = 1;

    match x {
        1 | 2 => println!("\t\tone or two"),
        3 => println!("\t\tthree"),
        _ => println!("\t\tanything"),
    }
    println!("\t----- End Multiple Patterns With Or -----\n");

    println!("\t----- Begin Matching Ranges -----");
    let x = 5;
    match x {
        1..=5 => println!("\t\tone through five"),
        _ => println!("\t\tsomething else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("\t\tearly ASCII letter"),
        'k'..='z' => println!("\t\tlate ASCII letter"),
        _ => println!("\t\tsomething else"),
    }

    println!("\t----- End Matching Ranges -----");
}
