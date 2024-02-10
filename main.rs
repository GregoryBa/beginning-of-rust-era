
use std::io; 

fn main() {
    let mut input_string = String::new(); // Create a mutable string to store the input

    println!("Please enter some text:");

    // Read a line of input from the standard input and store it in the input_string variable
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => {
            println!("You typed: {}", input_string.trim()); // Print the input, trimming any newline characters
        },
        Err(e) => {
            println!("Error reading input: {}", e); // Handle any potential errors
        }
    }
}