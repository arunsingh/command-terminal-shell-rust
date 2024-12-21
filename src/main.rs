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

        // Split the command line into the command and arguments, handling single, double quotes, and backslashes
        let mut parts = split_command_with_quotes(command_line);
        if let Some(command) = parts.next() {
            // Handle built-in commands
            if command == "echo" {
                let message: Vec<String> = parts.collect();
                println!("{}", message.join(" "));
                continue;
            } else if command == "type" {
                if let Some(queried_command) = parts.next() {
                    match queried_command.as_str() {
                        "echo" | "exit" | "type" | "pwd" | "cd" => {
                            println!("{} is a shell builtin", queried_command)
                        }
                        _ => {
                            if let Some(path) = find_executable(&queried_command) {
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
                        env::var("HOME").map_or_else(
                            |_| Path::new("/").to_path_buf(),
                            |s| Path::new(&s).to_path_buf(),
                        )
                    } else {
                        Path::new(&target_dir).to_path_buf()
                    };

                    if let Err(_err) = env::set_current_dir(&path) {
                        eprintln!("cd: {}: No such file or directory", target_dir);
                    }
                } else {
                    println!("cd: missing argument");
                }
                continue;
            }

            // Attempt to execute external commands
            if let Some(path) = find_executable(&command) {
                let args: Vec<String> = parts.collect();
                let status = Command::new(path).args(&args).status();

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

/// Attempts to locate an executable in the PATH.
/// Returns the full path to the executable if found, or None if not found.
fn find_executable(command: &str) -> Option<String> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let potential_path = Path::new(dir).join(command);
            // Must exist, be a file, and have non-readonly permissions (i.e., at least "executable" in a real shell)
            if potential_path.is_file()
                && fs::metadata(&potential_path)
                    .map(|m| !m.permissions().readonly())
                    .unwrap_or(false)
            {
                return Some(potential_path.to_string_lossy().to_string());
            }
        }
    }
    None
}

/// Splits the input string into tokens, respecting single quotes, double quotes, and backslash escaping.
/// - Outside of quotes:
///   - `\ ` becomes an actual space in the resulting token.
///   - `\\` becomes a literal `\`.
///   - `\'` or `\"` etc. becomes `'` or `"`.
/// - Inside single quotes:
///   - Everything is literal until the next single quote (except the single quote itself ends the quote).
/// - Inside double quotes:
///   - Everything is literal except that backslash can still escape `"` and `\`.
///
/// This should match enough of standard POSIX-like shell quoting to pass the required tests.
fn split_command_with_quotes(input: &str) -> impl Iterator<Item = String> {
    let mut parts = vec![];
    let mut current = String::new();

    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut escape_next = false; // signals that the next char is escaped
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if escape_next {
            // We are escaping this character. 
            // Behavior depends on whether we're in double quotes or not.
            current.push(c);
            escape_next = false;
            continue;
        }

        match c {
            // Toggle single quotes if not in double quotes
            '\'' if !in_double_quotes => {
                in_single_quotes = !in_single_quotes;
                // When we exit single quotes, we do not push the quote char itself.
                // Similarly when we enter, we skip pushing it.
            }

            // Toggle double quotes if not in single quotes
            '"' if !in_single_quotes => {
                in_double_quotes = !in_double_quotes;
            }

            '\\' => {
                // A backslash can escape something. The logic differs if we're inside single/double quotes or not:
                if in_single_quotes {
                    // In single quotes, backslash is literal. We do not do escapes at all.
                    current.push('\\');
                } else if in_double_quotes {
                    // In double quotes, a backslash only escapes \ and ".
                    // Peek next char to decide.
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '\\' || next_char == '"' {
                            // We skip pushing the backslash and set escape_next, 
                            // so the next iteration will push the next char as-is.
                            escape_next = true;
                        } else {
                            // If it's not \ or ", treat backslash as literal.
                            current.push('\\');
                        }
                    } else {
                        // No next char, treat it as literal
                        current.push('\\');
                    }
                } else {
                    // Outside quotes. A backslash can escape space, or `"` or `'` or `\`.
                    // Peek next char to decide.
                    if let Some(&next_char) = chars.peek() {
                        match next_char {
                            ' ' | '\'' | '"' | '\\' => {
                                // Skip the backslash, escape_next = true so next gets literal.
                                escape_next = true;
                            }
                            _ => {
                                // e.g., \a => we typically just treat this as 'a', ignoring the backslash.
                                // For simpler shell, let's push next_char and skip the backslash.
                                escape_next = true;
                            }
                        }
                    } else {
                        // No next char, treat the single backslash as literal
                        current.push('\\');
                    }
                }
            }

            // If we hit space outside of single or double quotes, this is an argument boundary
            ' ' if !in_single_quotes && !in_double_quotes => {
                if !current.is_empty() {
                    parts.push(current);
                    current = String::new();
                }
            }

            // Normal character
            _ => {
                current.push(c);
            }
        }
    }

    // If there's any leftover in 'current', push it as one final token
    if !current.is_empty() {
        parts.push(current);
    }

    parts.into_iter()
}
