use std::path::PathBuf;
use serde::{Serialize, Deserialize};

mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub user_id: u32,
    pub paused: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Projects {
    pub data: Vec<Project>,
    pub error: String,
}

impl Projects {
    pub fn get(&self, config_file: &PathBuf) {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);

        // Call api
        let endpoint = format!("{}{}", &config.url, "projects");
        let client = reqwest::blocking::Client::new();
        let res = client.get(endpoint)
            .header("Content-type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", &config.key)
            .send();

        match res {
            Ok(parsed) => {
                let projects = parsed.json::<Projects>().unwrap();
                for project in projects.data {
                    println!("➡️  ({id}) {name}",
                        id=project.id,
                        name=project.name);
                }
            }
            Err(e) => println!("Error happened: {}", e),
        }
    }
}
