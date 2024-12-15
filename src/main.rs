use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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
        let command_line = input.trim();

        // Handle empty input
        if command_line.is_empty() {
            continue; // If no command is entered, prompt again
        }

        // Handle the "exit" command
        if command_line == "exit 0" {
            break; // Exit the REPL loop
        }

        // Split the command line into the command and arguments
        let mut parts = command_line.split_whitespace();
        if let Some(command) = parts.next() {
            // Handle built-in commands
            if command == "echo" {
                let message: Vec<&str> = parts.collect();
                println!("{}", message.join(" "));
                continue;
            } else if command == "type" {
                if let Some(queried_command) = parts.next() {
                    match queried_command {
                        "echo" | "exit" | "type" | "pwd" | "cd" => println!("{} is a shell builtin", queried_command),
                        _ => {
                            if let Some(path) = find_executable(queried_command) {
                                println!("{} is {}", queried_command, path);
                            } else {
                                println!("{}: not found", queried_command);
                            }
                        }
                    }
                } else {
                    println!("type: missing argument");
                }
                continue;
            } else if command == "pwd" {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(err) => eprintln!("pwd: error retrieving current directory: {}", err),
                }
                continue;
            } else if command == "cd" {
                if let Some(target_dir) = parts.next() {
                    let path = if target_dir == "~" {
                        env::var("HOME").map_or_else(|_| Path::new("/").to_path_buf(), Path::new)
                    } else {
                        Path::new(target_dir).to_path_buf()
                    };

                    if let Err(err) = env::set_current_dir(&path) {
                        eprintln!("cd: {}: No such file or directory", target_dir);
                    }
                } else {
                    println!("cd: missing argument");
                }
                continue;
            }

            // Attempt to execute external commands
            if let Some(path) = find_executable(command) {
                let args: Vec<&str> = parts.collect();
                let status = Command::new(path)
                    .args(&args)
                    .status();

                match status {
                    Ok(exit_status) => {
                        if !exit_status.success() {
                            eprintln!("{}: command exited with status {}", command, exit_status);
                        }
                    }
                    Err(err) => {
                        eprintln!("{}: failed to execute: {}", command, err);
                    }
                }
            } else {
                println!("{}: command not found", command);
            }
        }
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
