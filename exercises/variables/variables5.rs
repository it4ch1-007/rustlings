// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut number: String = "T-H-R-E-E".to_string(); // Assign a string value
    println!("Spell a Number : {}", number);
    number = "3".to_string(); // Assign a string containing "3"
    let number_as_int: i32 = number.parse().expect("Parsing failed"); // Convert to an integer
    println!("Number plus two is : {}", number_as_int + 2)
}
