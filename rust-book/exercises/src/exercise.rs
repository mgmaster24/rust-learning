use std::collections::BTreeMap;

use crate::{
    collections, conditions_loops, employees, enums, functions, mut_immut, pointers, slices,
    structs, traits, variables,
};

pub struct Exercise {
    description: String,
    run: fn(),
}

pub fn build_exercises() -> BTreeMap<i32, Exercise> {
    let mut exercises: BTreeMap<i32, Exercise> = BTreeMap::new();
    exercises.insert(
        1,
        Exercise {
            description: String::from("Variables"),
            run: variables::exercise,
        },
    );
    exercises.insert(
        2,
        Exercise {
            description: String::from("Conditions And Loops"),
            run: conditions_loops::exercise,
        },
    );
    exercises.insert(
        3,
        Exercise {
            description: String::from("Functions"),
            run: functions::exercise,
        },
    );
    exercises.insert(
        4,
        Exercise {
            description: String::from("Mutable Vs. Immutable"),
            run: mut_immut::exercise,
        },
    );
    exercises.insert(
        5,
        Exercise {
            description: String::from("Slices"),
            run: slices::exercise,
        },
    );
    exercises.insert(
        6,
        Exercise {
            description: String::from("Structs"),
            run: structs::exercise,
        },
    );
    exercises.insert(
        7,
        Exercise {
            description: String::from("Enums"),
            run: enums::exercise,
        },
    );
    exercises.insert(
        8,
        Exercise {
            description: String::from("Collections"),
            run: collections::exercise,
        },
    );
    exercises.insert(
        9,
        Exercise {
            description: String::from("Employees"),
            run: employees::exercise,
        },
    );
    exercises.insert(
        10,
        Exercise {
            description: String::from("Traits"),
            run: traits::exercise,
        },
    );
    exercises.insert(
        11,
        Exercise {
            description: String::from("Pointers"),
            run: pointers::exercise,
        },
    );

    exercises
}

pub fn print_exercises(exercises: &BTreeMap<i32, Exercise>) {
    for (k, v) in exercises {
        println!("{}. {}", k, v.description);
    }
}

pub fn run_exercise(exercise: &Exercise) {
    println!(
        "---------------Running {} Exercise---------------",
        exercise.description
    );
    (exercise.run)();
    println!(
        "---------------Exiting {} Exercise---------------\n",
        exercise.description
    );
}
