use std::path::PathBuf;

#[path = "./config.rs"]
mod config;

pub fn get_projects(config_file: &PathBuf) {
    // Config
    let mut default_config = config::Config::default();
    let config = default_config.parse(&config_file);

    // Call api
    println!("Call API, Config: {:?}", config);

    // Return json result
    let projects = String::from("we");
    println!("{}", projects);
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
