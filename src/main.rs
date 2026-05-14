use std::env;
use std::process;

mod git_operation;
mod git_validation;

fn main() {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to get current directory: {e}");
            process::exit(1);
        }
    };

    match git_validation::validate_git_setup(&current_dir) {
        Ok(_) => {
            if git_operation::has_git_changes().is_ok() {
                git_operation::stash_changes();
                process::exit(0);
            }
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Git setup validation failed: {e}");
            process::exit(1);
        }
    }
}
