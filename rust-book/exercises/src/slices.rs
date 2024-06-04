pub fn exercise() {
    println!("---------------Running Slices Exercise---------------");
    // slices
    let word_str = String::from("This is a sentence with words");
    let first_word = find_first_word_in_string(&word_str);
    println!("first word: {}", first_word);

    println!("---------------Exiting Slices Exercise---------------\n");
}

fn find_first_word_in_string(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            let result = &s[0..i];
            return result;
        }
    }

    return &s[..];
}
