use std::io::{self, Write};

fn main() {
    loop {
        // Print the shell prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Read user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // Trim the input to remove extra whitespace or newline characters
        let command = input.trim();

        // Handle empty input
        if command.is_empty() {
            continue; // If no command is entered, prompt again
        }

        // Handle the "exit" command
        if command == "exit 0" {
            break; // Exit the REPL loop
        }

        // Handle the "echo" command
        if command.starts_with("echo ") {
            let message = &command[5..]; // Extract the part after "echo "
            println!("{}", message);
            continue; // Prompt again after handling echo
        }

        // For now, all other commands are treated as invalid
        println!("{}: command not found", command);
    }
}
