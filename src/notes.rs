use crate::{config::Config, tmux::TMUX};

pub struct NotesCommand {}

impl NotesCommand {
    pub fn run(cfg: &Config) {
        let dir = &cfg.notes.directory;
        if TMUX::window_exists("org", "notes") {
            TMUX::attach_or_select_window("org", "notes");
        } else {
            TMUX::create_window("org", "notes");
            TMUX::send_keys("org", "notes", &format!("cd {dir} && clear"));
            TMUX::watch_command(
                "org",
                "notes",
                cfg.notes.run_hidden,
                true,
                &cfg.notes.command,
            );
            TMUX::attach_or_select_window("org", "notes");
        }
    }
}
