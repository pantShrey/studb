use std::io::{self, Write};

fn print_prompt() {
    print!("studb > ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn read_input(buffer: &mut String) -> io::Result<usize> {
    buffer.clear();
    io::stdin().read_line(buffer)
}

fn main() {
    let mut buffer = String::new();

    loop {
        print_prompt();

        match read_input(&mut buffer) {
            Ok(0) => {
                // EOF detected
                println!();
                break;
            }
            Ok(_) => {
                let command = buffer.trim();
                if command == ".exit" {
                    break;
                } else if !command.is_empty() {
                    println!("Unrecognized command '{}'", command);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
