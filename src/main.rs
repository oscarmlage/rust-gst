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
use stamps::{Stamp, Stamps};

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

        // gst stamps [--project] [--last]
        ("stamps", Some(_matches)) => {
            console::info("List of stamps");
            let last = _matches.occurrences_of("last");
            let project = _matches.value_of("project")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            let stamps = Stamps {
                data: Vec::new(),
                error: "".to_string(),
            };
            stamps.get(&config_file, project, last);
        }

        // gst addtask --project NUM --title "title" --description "desc"
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
                err => println!("KO: {:?}, something happened", err),
            }
        }

        // gst stamp --start --task NUM --description "desc" --dstart "20120101" --dend "20120101"
        // gst stamp --stop
        // gst stamp --update --description "desc" --dstart "20120101" --dend "20120101"
        ("stamp", Some(_matches)) => {
            let start = _matches.occurrences_of("start");
            let stop = _matches.occurrences_of("stop");
            let update = _matches.occurrences_of("update");
            let task: u32 = _matches.value_of("task")
                .unwrap_or("0").trim().parse()
                .expect("Type a number!");
            let description: &str = _matches.value_of("description").unwrap_or("").trim();
            let dstart: &str = _matches.value_of("dstart").unwrap_or("").trim();
            let dend: &str = _matches.value_of("dend").unwrap_or("").trim();
            if start == 1 {
                console::info("Add a new stamp");
                match task {
                    0 => panic!("Need some task number to add the stamp"),
                    _ => {
                        let stamp = Stamp {
                            id: 0,
                            user_id: 0,
                            project_id: 0,
                            start: Some(dstart.to_string()),
                            end: Some(dend.to_string()),
                            description: Some(description.to_string()),
                            task_id: Some(task),
                        };
                        let added = stamp.add(&config_file);
                        match added.status() {
                            reqwest::StatusCode::OK => println!("OK"),
                            err => println!("KO: {:?}, something happened", err),
                        }
                    }
                }
            }
            else if stop == 1 {
                console::info("Stop last stamp");
                let stamp = Stamp {
                    id: 0,
                    user_id: 0,
                    project_id: 0,
                    start: Some("".to_string()),
                    end: Some("".to_string()),
                    description: Some("".to_string()),
                    task_id: Some(0),
                };
                let stopped = stamp.stop(&config_file);
                match stopped.status() {
                    reqwest::StatusCode::OK => println!("OK"),
                    err => println!("KO: {:?}, something happened", err),
                }
            }
            else if update == 1 {
                console::info("Update last stamp");
                // let stamp = {
                //     // description
                //     // dstart
                //     // dend
                // };
                // stamp.update();
            }
        }

        _ => console::error("Whut!!!"),
    }
}

