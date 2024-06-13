use crate::exercise::Exercise;

pub struct VariableExercise {}

impl Exercise for VariableExercise {
    fn exercise() {
        // variables
        println!("---------------Running Variables Exercise---------------");
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
        println!("---------------Exiting Variables Exercise---------------\n");
    }
}
