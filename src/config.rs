use serde::Deserialize;
use std::collections::HashMap;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub projects: HashMap<String, Project>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub session_name: String,
    pub directory: String,
    pub max_depth: u8,
    pub min_depth: u8,
    pub include_hidden: bool,
}

pub fn load_config() -> Config {
    const TEXT: &str = r#"

    [projects]
        [projects.personal]
        session_name = "kc"
        directory = "/home/kcaverly/personal"
        max_depth = 1
        min_depth = 1
        include_hidden = false

        [projects.work]
        session_name = "kc"
        directory = "/home/kcaverly/work"
        max_depth = 2
        min_depth = 2
        include_hidden = false

        [projects.dotfiles]
        session_name = "kc"
        directory = "/home/kcaverly/.dotfiles"
        max_depth = 3
        min_depth = 3
        include_hidden = false

    "#;

    let config: Config = toml::from_str(TEXT).unwrap();
    // println!("{:#?}", config);
    return config;
}
