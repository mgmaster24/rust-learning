mod common_collections;
mod concurrency;
mod cpc;
mod enums;
mod exercise;
mod input;
mod model;
mod mutex;
mod oop;
mod patterns;
mod pointers;
mod slices;
mod structs;
mod traits;
mod tree;
mod advanced_features;

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
                match chapters.get(&get_num_input(other)) {
                    Some(c) => {
                        exercise::run_chapter(c);
                    }
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
