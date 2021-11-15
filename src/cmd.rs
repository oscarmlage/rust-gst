use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[path = "./config.rs"]
mod config;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    id: u32,
    name: String,
    slug: String,
    description: Option<String>,
    status: String,
    user_id: u32,
    paused: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectsResponse {
    data: Vec<Project>,
    error: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    name: String,
    description: Option<String>,
    project_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TasksResponse {
    data: Vec<Task>,
    error: String,
}

pub fn api_call(config_file: &PathBuf, endpoint: String) -> Result<reqwest::blocking::Response, reqwest::Error> {
    // Config
    let mut default_config = config::Config::default();
    let config = default_config.parse(&config_file);

    // Call api
    let endpoint = format!("{}{}", &config.url, endpoint);
    let client = reqwest::blocking::Client::new();
    let res = client.get(endpoint)
        .header("Content-type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", &config.key)
        .send();
    res
}

pub fn get_projects(config_file: &PathBuf) {
    let response = api_call(&config_file, String::from("projects"));
    // eprintln!("{:#?}", response);
    match response {
        Ok(parsed) => {
            let projects = parsed.json::<ProjectsResponse>().unwrap();
            for project in projects.data {
                println!("➡️  ({id}) {name}",
                    id=project.id,
                    name=project.name);
            }
        }
        Err(e) => println!("Error happened: {}", e),
    }
}

pub fn get_tasks(config_file: &PathBuf, project: u32) {
    let response = api_call(&config_file, String::from("tasks"));
    // eprintln!("{:#?}", response);
    match response {
        Ok(parsed) => {
            // println!("{:?}", parsed);
            let tasks = parsed.json::<TasksResponse>().unwrap();
            for task in tasks.data {
                match project {
                    // 0 means no project, print them all
                    0 => {
                        println!("✳️  (pr: {projectid}) ({id}) {name}",
                        projectid=task.project_id,
                        id=task.id,
                        name=task.name);
                    },
                    // Otherwise but 0, print only task with that project.id
                    _ => {
                        if project == task.project_id {
                            println!("✳️  (pr: {projectid}) ({id}) {name}",
                            projectid=task.project_id,
                            id=task.id,
                            name=task.name);
                        }
                    }
                }
            }
        }
        Err(e) => println!("Error happened: {}", e),
    }
}
