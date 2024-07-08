use std::collections::BTreeMap;

use crate::{
    collections, conditions_loops, employees, enums, functions, mut_immut, pointers, slices,
    structs, traits, variables, tree, concurrency, mutex
};

use crate::input::{get_num_input, get_user_input};

pub struct Exercise {
    description: String,
    run: fn(),
}

pub struct Chapter {
    title: String,
    exercises: Vec<Exercise>
}

pub fn build_chapters() -> BTreeMap<i32, Chapter> {
    let mut chapters: BTreeMap<i32, Chapter> = BTreeMap::new();
    chapters.insert(3, Chapter{
        title: String::from("Common Programming Concepts"),
        exercises: vec![
            Exercise {
                description: String::from("Variables"),
                run: variables::exercise,
            },
            Exercise {
                description: String::from("Mutable Vs. Immutable"),
                run: mut_immut::exercise,
            },
            Exercise {
                description: String::from("Functions"),
                run: functions::exercise,
            },
            Exercise {
                description: String::from("Conditions And Loops"),
                run: conditions_loops::exercise,
            }
        ],
    });

    chapters.insert(4, Chapter {
        title: String::from("Understanding Ownership"),
        exercises: vec![
            Exercise {
                description: String::from("Slices"),
                run: slices::exercise,
            }
        ],
    });

    chapters.insert(5, Chapter {
        title: String::from("Using Structs to Structure Related Data"),
        exercises: vec![
            Exercise {
                description: String::from("Structs"),
                run: structs::exercise,
            },
        ],
    });

    chapters.insert(6, Chapter {
        title: String::from("Enums and Pattern Matching"),
        exercises: vec![
            Exercise {
                description: String::from("Enums"),
                run: enums::exercise,
            },
        ],
    });

    chapters.insert(8, Chapter {
        title: String::from("Common Collections"),
        exercises: vec![
            Exercise {
                description: String::from("Collections"),
                run: collections::exercise,
            },
            Exercise {
                description: String::from("Employees"),
                run: employees::exercise,
            },
        ],
    });

    chapters.insert(10, Chapter {
        title: String::from("Generic Types, Trait, and Lifetimes"),
        exercises: vec![
            Exercise {
                description: String::from("Traits"),
                run: traits::exercise,
            }
        ],
    });

    chapters.insert(15, Chapter {
        title: String::from("Smart Pointers"),
        exercises: vec![
            Exercise {
                description: String::from("Pointers"),
                run: pointers::exercise,
            },
            Exercise {
                description: String::from("Tree"),
                run: tree::exercise,
            },
        ],
    });

    chapters.insert(16, Chapter {
        title: String::from("Fearless Concurrency"),
        exercises: vec![
            Exercise {
                description: String::from("Concurrency"),
                run: concurrency::exercise,
            },
            Exercise {
                description: String::from("Mutexes"),
                run: mutex::exercise,
            }
        ],
    });

    
    chapters
}

pub fn print_chapters(chapters: &BTreeMap<i32, Chapter>) {
    for (k, v) in chapters {
        println!("{}. {}", k, v.title);
    }
}

pub fn run_chapter(chapter: &Chapter) {
    loop {
        println!("Please pick an exercise to to run: (or q to return to chapter select)");
        let mut i = 1;
        for e in &chapter.exercises {
            println!("{}. {}", i, e.description);
            i += 1;
        }

        match get_user_input().as_str() {
            "q" => break,
            "Q" => break,
            other => {
                println!("\n");
                run_exercise(chapter, (get_num_input(other.to_string()) - 1) as usize);
                println!("\n");
            }
        }
    }    
}

fn run_exercise(chapter: &Chapter, exercise_idx: usize) {
    let exercise = chapter.exercises.get(exercise_idx);
    match exercise {
        Some(e) => {
            println!(
                "---------------Running {} Exercise---------------",
                e.description
            );
            (e.run)();
            println!(
                "---------------Exiting {} Exercise---------------\n",
                e.description
            );
        },
        None => {
            println!("No exercise for that value.")
            
        }
    }
}
