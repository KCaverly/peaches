use crate::{config::Config, tmux::TMUX};

pub struct NotesCommand {}

impl NotesCommand {
    pub fn run(cfg: &Config) {
        let dir = &cfg.notes.directory;
        if TMUX::window_exists(&cfg.notes.session_name, "notes") {
            TMUX::attach_or_select_window(&cfg.notes.session_name, "notes");
        } else {
            TMUX::create_window(&cfg.notes.session_name, "notes");
            TMUX::send_keys(
                &cfg.notes.session_name,
                "notes",
                &format!("cd {dir} && clear"),
            );
            TMUX::watch_command(
                &cfg.notes.session_name,
                "notes",
                cfg.notes.run_hidden,
                true,
                &cfg.notes.command,
            );
            TMUX::attach_or_select_window(&cfg.notes.session_name, "notes");
        }
    }

}
