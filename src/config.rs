use casual;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub directories: HashMap<String, ProjectDirectory>,
    pub ssh: HashMap<String, SSH>,
    pub notes: Notes,
    pub tasks: Tasks,
}

impl Config {
    pub fn get_path() -> String {
        let mut peaches_path: String;
        if env::var("XDG_CONFIG_HOME").is_ok() {
            peaches_path = env::var("XDG_CONFIG_HOME").ok().unwrap();
        } else {
            peaches_path = env::var("HOME").ok().unwrap();
            peaches_path.push_str("/.peaches");
        }
        return peaches_path;
    }

    pub fn decrypt(encrypted: &str) -> String {
        if !encrypted.contains("crypt:") {
            return encrypted.to_string();
        }

        if env::var("PEACHES_KEY").is_ok() {
            let mc = new_magic_crypt!(env::var("PEACHES_KEY").ok().unwrap(), 256);
            return mc.decrypt_base64_to_string(encrypted).unwrap();
        } else {
            panic!("Please set encryption key as 'PEACHES_KEY' in environment variables.");
        }
    }

    pub fn encrypt(raw: &str) -> String {
        if env::var("PEACHES_KEY").is_ok() {
            let mc = new_magic_crypt!(env::var("PEACHES_KEY").ok().unwrap(), 256);
            return mc.encrypt_str_to_base64(raw);
        } else {
            panic!("Please set encryption key as 'PEACHES_KEY' in environment variables.");
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProjectDirectory {
    pub session_name: String,
    pub directory: String,
    pub max_depth: u8,
    pub min_depth: u8,
    pub include_hidden: bool,
}

#[derive(Debug, Deserialize)]
pub struct SSH {
    pub host: String,
    pub username: String,
    pub auth_method: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub session_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Notes {
    pub session_name: String,
    pub directory: String,
    pub command: String,
    pub run_hidden: bool,
}

pub fn generate_config() {
    const DEFAULT_CONFIG: &str = r#"
[directories]
    [directories.default]
    session_name = "default"
    directory = "/"
    min_depth = 1
    max_depth = 1
    include_hidden = false

[ssh]

    [ssh.default]
    host = "127.0.0.1"
    auth_method = "none"
    username = "testuser"
    password = "testpassword"

[tasks]
session_name = "org"

[notes]
session_name = "org"
directory = "~/notes"
command = "emanote"
run_hidden = true

"#;

    let peaches_path = Config::get_path();
    let path_clone = peaches_path.clone();

    if PathBuf::from(peaches_path).exists() {
        if casual::confirm(".peaches config already exists. Would you like to overwrite?") {
            fs::write(path_clone, DEFAULT_CONFIG).expect("Unable to write file");
        } else {
            println!("Closing without generating config!");
        }
    } else {
        fs::write(path_clone, DEFAULT_CONFIG).expect("Unable to write file");
    }
}

pub fn load_config() -> Config {
    let peaches_path = Config::get_path();

    let config_string: String = fs::read_to_string(peaches_path).ok().unwrap();

    let config: Config = toml::from_str(&config_string).unwrap();
    return config;
}
