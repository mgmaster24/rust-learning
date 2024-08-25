use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// Custom derive Macro
#[derive(HelloMacro)]
struct Pancakes;

// Standard way of defining traits.  Though using this method would force
// us to implement the hello_macro fn definition for each trait
// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!")
//     }
// }

pub fn exercise() {
    Pancakes::hello_macro();
}
