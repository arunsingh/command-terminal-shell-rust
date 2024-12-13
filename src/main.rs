use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

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

        // Handle the "type" command
        if command.starts_with("type ") {
            let queried_command = &command[5..]; // Extract the part after "type "
            match queried_command {
                "echo" | "exit" | "type" => println!("{} is a shell builtin", queried_command),
                _ => {
                    // Search for the command in the PATH
                    if let Some(path) = find_executable(queried_command) {
                        println!("{} is {}", queried_command, path);
                    } else {
                        println!("{}: not found", queried_command);
                    }
                }
            }
            continue; // Prompt again after handling type
        }

        // For now, all other commands are treated as invalid
        println!("{}: command not found", command);
    }
}

fn find_executable(command: &str) -> Option<String> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let potential_path = Path::new(dir).join(command);
            if potential_path.is_file() && fs::metadata(&potential_path).map(|m| m.permissions().readonly()).unwrap_or(false) == false {
                return Some(potential_path.to_string_lossy().to_string());
            }
        }
    }
    None
}
