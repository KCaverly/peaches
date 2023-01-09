use crate::{config::Config, tmux::TMUX};
use std::process;

pub struct TasksCommand {}

impl TasksCommand {
    pub fn run(cfg: &Config) {
        if TMUX::window_exists(&cfg.tasks.session_name, "tasks") {
            TMUX::attach_or_select_window(&cfg.tasks.session_name, "tasks");
        } else {
            TMUX::create_window(&cfg.tasks.session_name, "tasks");
            TMUX::send_keys(&cfg.tasks.session_name, "tasks", "taskwarrior-tui");
            TMUX::attach_or_select_window(&cfg.tasks.session_name, "tasks");
        }
    }

    pub fn healthcheck(verbose: bool) -> bool {
        if verbose {
            println!("\nRequirements for 'tasks':");
        }

        let requirements: Vec<String> = vec!["task".to_string(), "taskwarrior-tui".to_string()];
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
