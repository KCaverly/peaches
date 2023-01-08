use crate::{config::Config, tmux::TMUX};

pub struct TasksCommand {}

impl TasksCommand {
    pub fn run(cfg: &Config) {
        if TMUX::window_exists("tasks", "tasks") {
            TMUX::attach_or_select_window("tasks", "tasks");
        } else {
            TMUX::create_window("tasks", "tasks");
            TMUX::send_keys("tasks", "tasks", "taskwarrior-tui");
            TMUX::attach_or_select_window("tasks", "tasks");
        }
    }
}
