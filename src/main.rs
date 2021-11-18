use std::{process};
use std::path::PathBuf;

mod cli;
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

        // gst tasks [--project] [--last]
        ("tasks", Some(_matches)) => {
            console::info("List of tasks");
            let last = _matches.occurrences_of("last");
            let project = _matches.value_of("project")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            let tasks = Tasks {
                data: Vec::new(),
                error: "".to_string(),
            };
            tasks.get(&config_file, project, last);
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

        // gst addtask [--project] "title"
        ("addtask", Some(_matches)) => {
            console::info("Add a new task");
            let project: u32 = _matches.value_of("project")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            let title: &str = _matches.value_of("title")
                .unwrap_or("").trim();
            let description: &str = _matches.value_of("description")
                .unwrap_or("").trim();
            let task = Task {
                id: 0,
                name: title.to_string(),
                description: Some(description.to_string()),
                project_id: project,
                estimated: Some("1:00:00".to_string()),
            };
            // println!("{:?}", task);
            let added = task.add(&config_file);
            // println!("{:#?}", added);
            match added.status() {
                reqwest::StatusCode::OK => println!("OK"),
                other => println!("KO: {:?}, something happened", other),
            }
        }

        _ => console::error("Whut!!!"),
    }
}

