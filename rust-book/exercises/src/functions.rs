pub fn exercise() {
    // functions
    let s = String::from("Hello World");
    print_string(&s);
    println!("{s}");
}

fn print_string(some_str: &String) {
    println!("{some_str}");
}
