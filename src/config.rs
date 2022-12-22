use serde::Deserialize;
use std::{collections::HashMap, env, fs};
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

fn get_peaches_path() -> String {
    let mut peaches_path: String = env::var("HOME").ok().unwrap();
    peaches_path.push_str("/.peaches");
    if env::var("PEACHES_PATH").is_ok() {
        peaches_path = env::var("PEACHES_PATH").ok().unwrap();
    }

    return peaches_path;
}

pub fn generate_config() {
    const DEFAULT_CONFIG: &str = r#"
[projects]
    [projects.default]
    session_name = "default"
    directory = "~"
    min_depth = 1
    max_depth = 1
    include_hidden = false
"#;

    let peaches_path = get_peaches_path();
    fs::write(peaches_path, DEFAULT_CONFIG).expect("Unable to write file");
}

pub fn load_config() -> Config {
    let peaches_path = get_peaches_path();

    let config_string: String = fs::read_to_string(peaches_path).ok().unwrap();

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

    let config: Config = toml::from_str(&config_string).unwrap();
    // println!("{:#?}", config);
    return config;
}
