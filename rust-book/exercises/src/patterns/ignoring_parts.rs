pub fn exercise() {
    println!("\t---- Begin Ingore Value - Function Param ----");
    ignore_func_param(3, 4);
    println!("\t---- End Ingore Value - Function Param ----\n");

    println!("\t---- Begin Ingore Value - Nested Underscore  ----");
    let setting_value = Some(5);
    let new_val = Some(10);
    ignore_part_of_value(&setting_value, &new_val);
    println!("\t---- End Ingore Value - Nested Underscore ----\n");

    println!("\t---- Begin Ingore Value - Tuple Member  ----");
    ignore_value_tuple();
    println!("\t---- End Ingore Value - Tuple Member  ----\n");

    println!("\t---- Begin Ingore Value - Ranges  ----");
    ignoring_ranges();
    println!("\t---- End Ingore Value - Ranges  ----");
}

fn ignore_func_param(_: i32, y: i32) {
    println!("\t\tThis code only uses the y parameter: {y}");
}

fn ignore_part_of_value<'a>(mut setting_val: &'a Option<i32>, new_val: &'a Option<i32>) {
    match (setting_val, new_val) {
        (Some(_), Some(_)) => {
            println!("\t\tCan't overwrite an existing customized value");
        }
        _ => {
            setting_val = new_val;
        }
    }

    println!("\t\tsetting is {setting_val:?}");
}

fn ignore_value_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("\t\tSome numbers: {first}, {third}, {fifth}")
        }
    }
}

fn ignoring_ranges() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("\t\tx is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("\t\tSome numbers: {first}, {last}");
        }
    }
}
