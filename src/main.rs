use std::env;
use std::process;

mod git;

fn main() {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get current directory: {e}");
            process::exit(1);
        }
    };

    match git::validate_git_setup(&current_dir) {
        Ok(_) => {
            // not yet implemented
            println!("Validation completed successfully.");
            process::exit(0);
        }
        Err(e) => eprintln!("Git setup validation failed: {e}"),
    }
}
