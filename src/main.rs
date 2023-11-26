use std::path::PathBuf;



fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let matches = cd_cmd()
        .get_matches();
    let directory = matches
        .get_one::<PathBuf>("directory")
        .unwrap();
    std::env::set_current_dir(directory)?;
    Ok(())
}

fn cd_cmd() -> clap::Command {
    clap::Command::new("cd")
        .about("Change the current working directory")
        .args(&[
            clap::arg!(directory: <PATH> "New directory")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf))
        ])
}