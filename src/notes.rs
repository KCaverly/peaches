use crate::{config::Config, tmux::TMUX};

pub struct NotesCommand {}

impl NotesCommand {
    pub fn get_options() -> Vec<String> {
        return vec!["Launch notes".to_string()];
    }

    pub fn run(cfg: &Config) {
        let dir = &cfg.notes.directory;
        if TMUX::window_exists(&cfg.notes.session_name, "notes") {
            TMUX::attach_or_select_window(&cfg.notes.session_name, "notes");
        } else {
            TMUX::create_window(&cfg.notes.session_name, "notes");
            TMUX::send_keys(
                &cfg.notes.session_name,
                "notes",
                &format!("cd {dir} && nvim"),
            );
            TMUX::run_hidden_command(
                &cfg.notes.session_name,
                "notes",
                cfg.notes.run_hidden,
                true,
                &format!("cd {dir} && {0}", &cfg.notes.command),
            );
            TMUX::attach_or_select_window(&cfg.notes.session_name, "notes");
        }
    }
}
