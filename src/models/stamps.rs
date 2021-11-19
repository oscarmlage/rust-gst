use std::path::PathBuf;
use serde::{Serialize, Deserialize};

mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stamp {
    pub id: u32,
    pub user_id: u32,
    pub project_id: u32,
    pub start: Option<String>,
    pub end: Option<String>,
    pub description: Option<String>,
    pub task_id: Option<u32>,
}

impl Stamp {
    pub fn new(config_file: &PathBuf,
               dstart: &str,
               dend: &str,
               description: &str,
               task: u32
            ) -> Stamp {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);
        Stamp {
            id: 0,
            user_id: config.user_id,
            project_id: 0,
            start: Some(dstart.to_string()),
            end: Some(dend.to_string()),
            description: Some(description.to_string()),
            task_id: Some(task),
        }
    }

    #[tokio::main]
    pub async fn api_post(&self, config_file: &PathBuf, endpoint: &str) -> reqwest::Response {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);

        // Call api
        let endpoint = format!("{}{}", &config.url, String::from(endpoint));
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
pub struct Stamps {
    pub data: Vec<Stamp>,
    pub error: String,
}

impl Stamps {
    pub fn get(&self, config_file: &PathBuf, project: u32, last: u64) {
        // Config
        let mut default_config = config::Config::default();
        let config = default_config.parse(&config_file);

        // Call api
        let endpoint = format!("{}{}", &config.url, "stamps");
        let client = reqwest::blocking::Client::new();
        let res = client.get(endpoint)
            .header("Content-type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", &config.key)
            .send();

        match res {
            Ok(parsed) => {
                let stamps = parsed.json::<Stamps>().unwrap();
                // eprintln!("{:#?}", stamps);
                for stamp in stamps.data {
                    match project {
                        0 => {
                            println!("⏳ ({id}) {description}",
                                id=stamp.id,
                                description=stamp.description.unwrap());
                            if last != 0 { break; }
                        },

                        _ => {
                            if project == stamp.project_id {
                                println!("⏳ ({id}) {description}",
                                    id=stamp.id,
                                    description=stamp.description.unwrap());
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
