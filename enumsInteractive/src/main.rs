use std::process::Command;
use std::io;


//good syntax
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    // This function receives a FileOperation enum and runs the corresponding system command using Command::new()

    match operation {
        FileOperation::List(directory_path) => {
            Command::new("ls") // run the 'ls' command
                .arg(directory_path) // pass the directory path as an argument
                .status() //exe command
                .expect("Failed to execute ls");
        }

        FileOperation::Display(file_path) => {
            Command::new("cat") // run the 'cat' command
                .arg(file_path)
                .status()
                .expect("Failed to execute cat");
        }

        FileOperation::Create(file_path, content) => {
            let command = format!("echo '{}' > {}", content, file_path);

            Command::new("sh")
                .arg("-c") // tells the shell to execute the command string
                .arg(command)
                .status()
                .expect("Failed to create file");

            println!("File created successfully.");
        }

        FileOperation::Remove(file_path) => {
            Command::new("rm")
                .arg(file_path)
                .status()
                .expect("Failed to remove file");

            println!("File removed.");
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

// Helper function to read user input from the terminal
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}


//---------------------------------------------------------------------------------------------------
//------------------------------------------------


fn main() {


    println!("enums!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("Enter your choice (0-5):");

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                println!("Enter directory path:");
                let path = read_input();
                perform_operation(FileOperation::List(path));
            }

            "2" => {
                println!("Enter file path:");
                let file = read_input();
                perform_operation(FileOperation::Display(file));
            }

            "3" => {
                println!("Enter file path:");
                let file = read_input();

                println!("Enter content:");
                let content = read_input();

                perform_operation(FileOperation::Create(file, content));
            }

            "4" => {
                println!("Enter file path:");
                let file = read_input();
                perform_operation(FileOperation::Remove(file));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Try again."); //
            }
        }
    }
    
}