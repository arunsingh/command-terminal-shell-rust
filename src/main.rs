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

        // Handle invalid commands
        if command.is_empty() {
            continue; // If no command is entered, prompt again
        }

        // For now, all commands are treated as invalid
        println!("{}: command not found", command);

        // Exit after handling the command (as specified for this stage)
        break;
    }
}
