use std::process;

use crate::{fuzzy_finder, tmux::TMUX};

pub struct DockerCommand {}

impl DockerCommand {
    pub fn get_options() -> Vec<String> {
        let ls = process::Command::new("docker")
            .args(vec!["ps"])
            .output()
            .unwrap();
        let output = String::from_utf8(ls.stdout);
        let mut container_names: Vec<String> = Vec::new();
        for line in output.expect("IS DOCKER INSTALLED?").split("\n") {
            let name = line.split("   ").last().unwrap().trim();
            if (name.len() > 0) & (name != "NAMES") {
                container_names.push(name.to_string());
            }
        }

        return container_names;
    }

    pub fn post_search_command(selected: &str) {
        let name = &selected.replace("-", "_");

        // Launch Project
        // If window exists, presume it has already activated the docker container
        if TMUX::window_exists("docker", name) {
            TMUX::attach_or_select_window("docker", name);
        } else {
            // Otherwise, create the window and enter a bash shell in the docker container.
            TMUX::create_window("docker", name);
            TMUX::send_keys("docker", name, &format!("docker exec -ti {selected}"));
            TMUX::attach_or_select_window("docker", name);
        }
    }

    pub fn run() {
        let options = Self::get_options();
        let selected = fuzzy_finder::search_options(options);
        Self::post_search_command(&selected);
    }

    pub fn healthcheck(verbose: bool) -> bool {
        if verbose {
            println!("\nRequirements for 'docker':");
        }

        let requirements: Vec<String> = vec!["docker".to_string()];
        for req in requirements.iter() {
            let c = process::Command::new("which")
                .arg(req)
                .stdout(process::Stdio::null())
                .status()
                .unwrap();

            if !c.success() {
                println!("{}     Missing...", req);
                return false;
            } else {
                if verbose {
                    println!("{}     Found...", req);
                }
            }
        }
        return true;
    }
}
