use crate::input::get_user_input;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn exercise() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i *= 2;
        println!("{i}");
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(3.14),
        SpreadSheetCell::Text(String::from("PI")),
    ];

    for cell in row {
        match cell {
            SpreadSheetCell::Float(f) => {
                println!("Float value: {}", f)
            }
            SpreadSheetCell::Int(i) => {
                println!("Integer value: {}", i)
            }
            SpreadSheetCell::Text(str) => {
                println!("Text value: {}", str)
            }
        }
    }

    let mut nums: Vec<i32> = vec![1, 42, 33, 24, 55, 70, 8, 99, 121, 24, 42, 11, 1, 3, 24, 65];
    println!("Median of nums vector: {}", median(&mut nums));
    println!("Mode of nums vector: {}", mode(&mut nums));

    println!("Please enter a string to convert to pig latin followed by the return (enter) key:");
    let input = get_user_input();
    let converted = convert_to_pig_latin(input);
    println!("Your converted string is:\n{}", converted);
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let len = v.len();
    let half_len = len / 2;
    println!("half len {}", half_len);
    let mid = v[half_len];
    if len % 2 == 0 {
        return (mid + v[half_len + 1]) / 2;
    }

    mid
}

fn mode(v: &mut Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut nums = HashMap::new();
    for n in v {
        let count = nums.entry(n).or_insert(0);
        *count += 1;
    }

    let mut largest_key: i32 = 0;
    let mut largest_val: i32 = 0;
    for (k, v) in nums {
        if v > largest_val {
            largest_val = v;
            largest_key = *k;
        }
    }

    largest_key
}

fn convert_to_pig_latin(str: String) -> String {
    let mut words = Vec::new();
    for word in str.split_whitespace() {
        // take first char
        let first_char = word
            .chars()
            .nth(0)
            .expect("Word should have characters do to use of split");
        if vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'].contains(&first_char) {
            words.push(format!("{}{}", word, "hay"));
        } else {
            words.push(format!(
                "{}{}",
                &word[1..],
                format!("{}{}", first_char, "ay")
            ));
        }
    }

    words.join(" ")
}
