use std::{collections::HashMap, io, process};
use walkdir::{DirEntry, WalkDir};

use crate::{
    config::{Config, ProjectDirectory},
    fuzzy_finder,
    tmux::TMUX,
};

pub struct DirsCommand {}

impl DirsCommand {
    fn get_folders(path: &str, min_depth: u8, max_depth: u8) -> Result<Vec<DirEntry>, io::Error> {
        Ok(WalkDir::new(path)
            .min_depth(min_depth.into())
            .max_depth(max_depth.into())
            .into_iter()
            .filter(|x| x.is_ok())
            .map(|r| r.unwrap())
            .filter(|x| x.path().is_dir())
            .collect::<Vec<DirEntry>>())
    }

    pub fn get_options(directories: &HashMap<String, ProjectDirectory>) -> Vec<String> {
        let mut file_list: Vec<String> = Vec::new();

        for (_key, value) in &*directories {
            let dir_files: Vec<String> =
                Self::get_folders(&value.directory, value.min_depth, value.max_depth)
                    .unwrap()
                    .iter()
                    .map(|x| x.path().to_str().unwrap().to_string())
                    .filter(|x| {
                        if value.include_hidden {
                            true
                        } else {
                            !x.starts_with(".")
                        }
                    })
                    .map(|x| x.to_string())
                    .filter(|x| !x.contains(".git/"))
                    .filter(|x| !x.contains("bin"))
                    .collect();

            for file in dir_files.iter() {
                if file.to_string() != value.directory {
                    file_list.push(file.to_string());
                }
            }
        }
        return file_list;
    }

    fn get_directory_details<'a>(cfg: &'a Config, selected: &'a str) -> &'a ProjectDirectory {
        for (_key, value) in &cfg.directories {
            if selected.contains(&value.directory) {
                return value;
            }
        }
        panic!("No project found for directory")
    }

    pub fn post_search_command(cfg: &Config, selected: &str) {
        let details = Self::get_directory_details(cfg, selected);
        let name = selected.split("/").last().unwrap();

        TMUX::create_window(&details.session_name, name);
        TMUX::send_keys(
            &details.session_name,
            name,
            &format!("cd {selected} && clear"),
        );
        TMUX::attach_or_select_window(&details.session_name, name);
    }

    pub fn run(cfg: &Config) {
        let options = Self::get_options(&cfg.directories);
        let selected = fuzzy_finder::search_options(options);
        Self::post_search_command(&cfg, &selected);
    }

    pub fn healthcheck(verbose: bool) -> bool {
        if verbose {
            println!("\nRequirements for 'dirs':");
        }

        let requirements: Vec<String> = vec!["tmux".to_string()];
        for req in requirements.iter() {
            let c = process::Command::new("which")
                .arg(req)
                .stdout(process::Stdio::null())
                .status()
                .unwrap();

            if !c.success() {
                println!("{}      Missing...", req);
                return false;
            } else {
                if verbose {
                    println!("{}      Found...", req);
                }
            }
        }
        return true;
    }
}
