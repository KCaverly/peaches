use crate::{config::Config, tmux::TMUX};

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
}
