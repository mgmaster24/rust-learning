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

use exercise::Exercise;
use input::{get_num_input, get_user_input};
use variables::VariableExercise;

#[derive(PartialEq, Eq, Hash, Debug)]
enum ExerciseModule {
    ConditionsLoops,
    Variables,
    Functions,
    MutImmut,
    Slices,
    Structs,
    Enums,
    Collections,
    Employees,
    Traits,
    Pointers,
}

fn main() {
    use std::collections::HashMap;
    let mut exercises: HashMap<ExerciseModule, fn()> = HashMap::new();
    exercises.insert(ExerciseModule::Variables, VariableExercise::exercise);
    exercises.insert(ExerciseModule::ConditionsLoops, conditions_loops::exercise);
    exercises.insert(ExerciseModule::Functions, functions::exercise);
    exercises.insert(ExerciseModule::MutImmut, mut_immut::exercise);
    exercises.insert(ExerciseModule::Slices, slices::exercise);
    exercises.insert(ExerciseModule::Structs, structs::exercise);
    exercises.insert(ExerciseModule::Enums, enums::exercise);
    exercises.insert(ExerciseModule::Collections, collections::exercise);
    exercises.insert(ExerciseModule::Employees, employees::exercise);
    exercises.insert(ExerciseModule::Traits, traits::exercise);
    exercises.insert(ExerciseModule::Pointers, pointers::exercise);
    loop {
        let mut choice_to_exercise = HashMap::new();
        println!("Please pick a exercise to run: (or q to exit)");
        let mut index: i32 = 1;
        for key in exercises.keys() {
            choice_to_exercise.insert(index, key);
            let exercise_str = match key {
                ExerciseModule::ConditionsLoops => "Conditions And Loops".to_string(),
                ExerciseModule::MutImmut => "Mutable vs Immutable".to_string(),
                _ => format!("{:?}", key),
            };

            println!("{}. {}", index, exercise_str);
            index += 1;
        }

        let exercise = get_user_input();
        match exercise.as_str() {
            "q" => break,
            "Q" => break,
            _ => {
                println!("\n");

                let ex_num = get_num_input(exercise);
                if ex_num < 1 || ex_num as usize > exercises.len() {
                    println!("Please enter a value from the list above");
                    continue;
                }

                match choice_to_exercise.get(&ex_num) {
                    Some(em) => {
                        match exercises.get(em) {
                            Some(f) => f(),
                            None => {
                                println!("No exercise found for this module");
                                continue;
                            }
                        };
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
