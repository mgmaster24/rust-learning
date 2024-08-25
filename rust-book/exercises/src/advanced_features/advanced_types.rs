pub fn exercise() {
    type_synonyms();
    function_ptrs();
    returning_closures();
}

fn type_synonyms() {
    println!("\t----- Begin Type Synonyms -----");
    // This is a simple example.  Type Synonyms are more generally used for defining lengthy types. So as to
    // make writing the tyoe in multiple places easier to read
    type Kilometers = i32;
    let x: Kilometers = 42;
    println!("\t\tNum Kilometers: {x}");
    println!("\t----- End Type Synonyms -----\n");
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn function_ptrs() {
    println!("\t----- Begin Function Pointers -----");
    let answer = do_twice(add_one, 5);
    assert_eq!(12, answer);
    println!("\t\tExample using a function pointer to add one to five twice: {answer}");

    let list_of_nums = vec![1, 2, 3];
    let list_of_strs_closure: Vec<String> = list_of_nums.iter().map(|i| i.to_string()).collect();
    let list_of_str_fnptr: Vec<String> = list_of_nums.iter().map(ToString::to_string).collect();

    println!(
        "\t\tExample of using a closure in a map {:?}",
        list_of_strs_closure
    );
    println!(
        "\t\tExample of using a Function Pointer in a map {:?}",
        list_of_str_fnptr
    );

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("\t\tExample of using an enum initializer function as closure to map.");
    println!("\t\tList of statuses: {:?}", list_of_statuses);
    println!("\t----- End Function Pointers -----\n");
}

fn returning_closures() {
    println!("\t----- Begin Returning Closures -----");
    let closure = get_closure();
    let val = closure(5);
    println!("\t\tExample of getting a closure from a function and using it to get a value: {val}");
    assert_eq!(6, val);
    println!("\t----- End Returning Closures -----");
}

fn get_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
