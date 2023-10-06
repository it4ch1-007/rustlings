// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut shopping_list: Vec<&'static str> = Vec::new();
    //THE STATIC DENOTES THAT THE VARIABLE INPUT ARGUMENT IS AVAILABLE FOR WHOLE TIME FOR THE FUNCTION
    shopping_list.push("milk");
}
