use std::{process};
use std::path::PathBuf;

mod cli;
mod cmd;
pub mod console;

fn main() {
    // Get cli matches
    let matches = cli::build_cli().get_matches();

    // Read the config file
    let config_file = match matches.value_of("config") {
        Some(path) => PathBuf::from(path),
        None => {
            console::error("Error, not able to load config");
            process::exit(1);
        }
    };

    // Start program flow
    match matches.subcommand() {
        ("projects", Some(_matches)) => {
            console::info("List of projects");
            cmd::get_projects(&config_file);
        }

        ("tasks", Some(matches)) => {
            let project = matches.value_of("project").unwrap_or("");
            println!("Tasks");
            if !project.is_empty() {
                println!("Project: {}", project);
            }
        }

        _ => console::error("Whut!!!"),
    }
}

