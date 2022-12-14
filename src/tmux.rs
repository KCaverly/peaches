pub struct TMUX {}
use std::{env, process, str};

impl TMUX {
    pub fn in_tmux() -> bool {
        return env::var("TMUX").is_ok();
    }

    pub fn active() -> bool {
        if Self::in_tmux() {
            return true;
        }

        let output: process::Output = process::Command::new("tmux")
            .args(vec!["-t", "/dev/pts/1"])
            .output()
            .unwrap()
            .into();

        if str::from_utf8(&output.stdout)
            .unwrap()
            .contains("no server")
        {
            return false;
        } else {
            return true;
        }
    }

    pub fn session_exists(session_name: &str) -> bool {
        // If tmux is not active, session_name cant exist
        if !Self::active() {
            return false;
        }

        let output: process::Output = process::Command::new("tmux")
            .args(vec!["has-session", "-t", session_name])
            .output()
            .unwrap()
            .into();

        if str::from_utf8(&output.stdout)
            .unwrap()
            .contains("can't find session")
        {
            return false;
        }

        return true;
    }

    pub fn window_exists(session_name: &str, window_name: &str) -> bool {
        // In order for window to exist, session must exist
        if !Self::session_exists(session_name) {
            return false;
        }

        let output: process::Output = process::Command::new("tmux")
            .args(vec![
                "has-session",
                "-t",
                &format!("{session_name}:{window_name}"),
            ])
            .output()
            .unwrap()
            .into();

        if str::from_utf8(&output.stdout)
            .unwrap()
            .contains("can't find window")
        {
            return false;
        }

        return true;
    }
}
