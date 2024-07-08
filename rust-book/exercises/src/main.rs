mod collections;
mod conditions_loops;
mod employees;
mod enums;
mod exercise;
mod functions;
mod input;
mod mut_immut;
mod pointers;
mod slices;
mod structs;
mod traits;
mod variables;
mod tree;
mod concurrency;
mod mutex;

use input::{get_num_input, get_user_input};

fn main() {
    let chapters = exercise::build_chapters();
    loop {
        println!("Please pick a chapter to see the available exercises to run: (or q to exit)");
        exercise::print_chapters(&chapters);
        match get_user_input().as_str() {
            "q" => break,
            "Q" => break,
            other => {
                println!("\n");
                match chapters.get(&get_num_input(other.to_string())) {
                    Some(c) => {
                        exercise::run_chapter(c);
                    },
                    None => {
                        println!("No choice for that value. Please choose again!");
                        continue;
                    }
                };

                println!("\n")
            }
        }
    }
}
