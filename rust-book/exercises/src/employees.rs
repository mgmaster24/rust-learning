use crate::input::get_user_input;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Department {
    Engineering,
    ProductDevelopment,
    Sales,
    IT,
    HR,
}

pub fn exercise() {
    let mut employees: HashMap<Department, Vec<String>> = HashMap::new();

    loop {
        println!("Please choose and option below.");
        println!("1. Add an Employee");
        println!("2. List Employees by Department");
        println!("q. Quit");

        let choice = get_user_input();
        match choice.as_str() {
            "1" => add_employee(&mut employees),
            "2" => list_employees_by_department(&mut employees),
            "q" => break,
            "Q" => break,
            _ => println!("Choice not recognized. Please try again."),
        }
    }
}

fn add_employee(employees: &mut HashMap<Department, Vec<String>>) {
    println!("Please enter the employees name:");
    let name = get_user_input();
    let department: Department;

    loop {
        println!("Please choose a department from the list below:");
        let mut i: usize = 1;
        let mut choices = HashMap::new();
        for dept in vec![
            Department::Engineering,
            Department::ProductDevelopment,
            Department::Sales,
            Department::IT,
            Department::HR,
        ] {
            let display: String = match dept {
                Department::ProductDevelopment => String::from("Product Development"),
                _ => format!("{:?}", dept),
            };
            println!("{}. {}", i, display);
            choices.insert(i, dept);
            i += 1;
        }
        let input = get_user_input();
        let dept_num: usize = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a department number.");
                continue;
            }
        };

        department = match choices.get(&dept_num).unwrap() {
            Department::Engineering => Department::Engineering,
            Department::ProductDevelopment => Department::ProductDevelopment,
            Department::Sales => Department::Sales,
            Department::IT => Department::IT,
            Department::HR => Department::HR,
        };

        employees
            .entry(department)
            .or_insert(Vec::new())
            .push(name.to_string());
        break;
    }
}

fn list_employees_by_department(employees: &HashMap<Department, Vec<String>>) {
    for (dept, names) in employees {
        println!("{:?}", dept);
        println!("----------------------");
        for name in names {
            println!("{}", name);
        }
    }
}
