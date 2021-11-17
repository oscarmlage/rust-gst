use std::{process};
use std::path::PathBuf;

mod cli;
mod cmd;
pub mod console;

#[path = "./models/tasks.rs"]
mod tasks;
use tasks::{Task, Tasks};

#[path = "./models/projects.rs"]
mod projects;
use projects::Projects;

#[path = "./models/stamps.rs"]
mod stamps;
use stamps::Stamps;

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
            let projects = Projects {
                data: Vec::new(),
                error: "".to_string(),
            };
            projects.get(&config_file);
        }

        // gst tasks [--project]
        ("tasks", Some(_matches)) => {
            console::info("List of tasks");
            let project = _matches.value_of("project")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            let tasks = Tasks {
                data: Vec::new(),
                error: "".to_string(),
            };
            tasks.get(&config_file, project);
        }

        // gst stamps [--last]
        ("stamps", Some(_matches)) => {
            console::info("List of stamps");
            let last = _matches.occurrences_of("last");
            let stamps = Stamps {
                data: Vec::new(),
                error: "".to_string(),
            };
            stamps.get(&config_file, last);
        }

        _ => console::error("Whut!!!"),
    }
}

