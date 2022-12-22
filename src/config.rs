use serde::Deserialize;
use std::collections::HashMap;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    projects: HashMap<String, Project>,
}

#[derive(Debug, Deserialize)]
struct Project {
    session_name: String,
    directory: String,
    max_depth: u8,
    min_depth: u8,
    include_hidden: bool
}

pub fn load_config() {
    const TEXT: &str = r#"
    version = "0.0"

    [projects]
        [projects.personal]
        session_name = "kc"
        directory = "~/personal"
        max_depth = 1
        min_depth = 1
        include_hidden = false
    "#;


    let config: Config = toml::from_str(TEXT).unwrap();
    println!("{:#?}", config);

}
