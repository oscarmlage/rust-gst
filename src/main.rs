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

    // Command matcher
    match matches.subcommand() {
        // gst projects
        ("projects", Some(_matches)) => {
            console::info("List of projects");
            cmd::get_projects(&config_file);
        }

        // gst tasks [--project]
        ("tasks", Some(_matches)) => {
            console::info("List of tasks");
            let project = _matches.value_of("project")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            cmd::get_tasks(&config_file, project);
        }

        _ => console::error("Whut!!!"),
    }
}

