pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}

pub fn get_num_input(input: String) -> i32 {
    let val: i32 = input.parse().unwrap_or_else(|_| {
        println!("Please enter a department number.");
        -1
    });

    val
}
