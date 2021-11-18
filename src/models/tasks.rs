use std::path::PathBuf;
use serde::{Serialize, Deserialize};

mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub project_id: u32,
    pub estimated: Option<String>,
}

impl Task {
    #[tokio::main]
    pub async fn add(&self, config_file: &PathBuf) -> reqwest::Response {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);

        // Call api
        let endpoint = format!("{}{}", &config.url, "task/add");
        let client = reqwest::Client::new();
        let res = client.post(endpoint)
            .header("Authorization", &config.key)
            .json(&self)
            .send()
            .await;

        res.unwrap()
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub data: Vec<Task>,
    pub error: String,
}

impl Tasks {
    pub fn get(&self, config_file: &PathBuf, project: u32, last: u64) {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);

        // Call api
        let endpoint = format!("{}{}", &config.url, "tasks");
        let client = reqwest::blocking::Client::new();
        let res = client.get(endpoint)
            .header("Content-type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", &config.key)
            .send();
        match res {
            Ok(parsed) => {
                // println!("{:?}", parsed);
                let tasks = parsed.json::<Tasks>().unwrap();
                for task in tasks.data {
                    match project {
                        // 0 means no project, print them all
                        0 => {
                            println!("✳️  (pr: {projectid}) ({id}) {name}",
                            projectid=task.project_id,
                            id=task.id,
                            name=task.name);
                            if last != 0 { break; }
                        },
                        // Otherwise but 0, print only task with that project.id
                        _ => {
                            if project == task.project_id {
                                println!("✳️  (pr: {projectid}) ({id}) {name}",
                                projectid=task.project_id,
                                id=task.id,
                                name=task.name);
                                if last != 0 { break; }
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Error happened: {}", e),
        }
    }
}

