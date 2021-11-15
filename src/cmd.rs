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
    let projects_json = api_call(&config_file, String::from("stamps"));
    println!("{:?}", projects_json);
}

pub fn get_tasks(config_file: &PathBuf, project: i32) {
    // Config
    let mut default_config = config::Config::default();
    let config = default_config.parse(&config_file);

    // Call api
    println!("Call API, Config: {:?}", config);

    // Return json result
    let tasks = String::from("tasks");
    println!("{}", tasks);

    println!("{}", project);
}
