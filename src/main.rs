
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Parse command line arguments
    let matches = cd_cmd()
        .get_matches();

    // Get the directory argument from the command line
    let directory = matches
        .get_one::<PathBuf>("directory")
        .unwrap();

    // Change the current working directory to the specified directory
    std::env::set_current_dir(directory)?;

    // Return success
    Ok(())
}

fn cd_cmd() -> clap::Command {
    // Create a new command line argument parser
    clap::Command::new("cd")
        .about("Change the current working directory")
        .args(&[
            // Add an argument for the new directory
            clap::arg!(directory: <PATH> "New directory")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
        ])
}
//
//In this code, we define a command line argument parser using the `clap` crate. The `cd_cmd` function creates a new `Command` object and adds an argument for the new directory.
//
//The `main` function first parses the command line arguments using the `get_matches` method. Then, it retrieves the new directory argument from the parsed arguments.
//
//Next, the `main` function changes the current working directory to the specified directory using the `std::env::set_current_dir` function.
//
//Finally, the `main` function returns success..</s>