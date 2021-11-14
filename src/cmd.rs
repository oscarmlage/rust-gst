use std::path::PathBuf;

#[path = "./config.rs"]
mod config;

pub fn api_call(config_file: &PathBuf, endpoint: String) -> String {
    // Config
    let mut default_config = config::Config::default();
    let config = default_config.parse(&config_file);

    // Call api
    let endpoint = format!("{}{}", &config.url, endpoint);
    let client = reqwest::blocking::Client::new();
    let res = client.get(endpoint)
        .header("Authorization", &config.key)
        .send();

    match res {
        Ok(res) => return res.text().unwrap(),
        Err(e) => panic!("Problem with endpoint: {:?}", e),
    };
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
