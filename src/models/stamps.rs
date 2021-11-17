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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stamps {
    pub data: Vec<Stamp>,
    pub error: String,
}

impl Stamps {
    pub fn get(&self, config_file: &PathBuf, last: u64) {
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
                    println!("⏳ ({id}) {description}",
                        id=stamp.id,
                        description=stamp.description.unwrap());
                    if last != 0 { break; }
                }
            }
            Err(e) => println!("Error happened: {}", e),
        }
    }
}
