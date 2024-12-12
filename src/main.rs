#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Print the shell prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Trim the input and check if it is empty
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Exit the shell on "exit" command
        if input == "exit" {
            println!("Exiting shell...");
            break;
        }

        // Check for valid commands (in this simple example, only "hello" is valid)
        match input {
            "hello" => println!("Hello, world!"),
            _ => println!("Error: '{}' is not a valid command", input),
        }
    }
}
