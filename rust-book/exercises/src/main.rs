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

use input::{get_num_input, get_user_input};

fn main() {
    let exercises = exercise::build_exercises();
    loop {
        println!("Please pick a exercise to run: (or q to exit)");
        exercise::print_exercises(&exercises);
        match get_user_input().as_str() {
            "q" => break,
            "Q" => break,
            other => {
                println!("\n");
                match exercises.get(&get_num_input(other.to_string())) {
                    Some(e) => exercise::run_exercise(e),
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
