pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

pub fn get_num_input(input: &str) -> i32 {
    let val: i32 = input.parse().unwrap_or_else(|_| -1);

    val
}

pub fn execute_or_quit(func: fn(str: &str)) -> bool {
    match get_user_input().as_str() {
        "q" => false,
        "Q" => false,
        other => {
            println!("\n");
            func(other);
            println!("\n");
            true
        }
    }
}
