use std::{env, process, str};
pub struct TMUX {}

impl TMUX {
    pub fn in_tmux() -> bool {
        return env::var("TMUX").is_ok();
    }

    pub fn run_tmux_command(args: Vec<&str>) -> bool {
        let status = process::Command::new("tmux")
            .args(args)
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .status()
            .unwrap();
        return status.success();
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

        return Self::run_tmux_command(vec!["has-session", "-t", session_name]);
    }

    pub fn window_exists(session_name: &str, window_name: &str) -> bool {
        // In order for window to exist, session must exist
        if !Self::session_exists(session_name) {
            return false;
        }

        return Self::run_tmux_command(vec![
            "has-session",
            "-t",
            &format!("{session_name}:{window_name}"),
        ]);
    }

    pub fn create_window(session_name: &str, window_name: &str) -> bool {
        // Do not recreate window if one exists
        if Self::window_exists(session_name, window_name) {
            return true;
        }

        // If session exists, do not recreate Session
        if Self::session_exists(session_name) {
            return Self::run_tmux_command(vec![
                "new-window",
                "-t",
                session_name,
                "-n",
                window_name,
            ]);
        }

        // Create Session and Window in Detached State
        return Self::run_tmux_command(vec![
            "new-session",
            "-d",
            "-s",
            session_name,
            "-n",
            window_name,
        ]);
    }

    pub fn attach_or_select_window(session_name: &str, window_name: &str) -> bool {
        if !Self::window_exists(session_name, window_name) {
            panic!("Window Doesnt Exist!, Please create window first!");
        }

        if Self::in_tmux() {
            let _o = process::Command::new("tmux")
                .args(vec![
                    "switch",
                    "-t",
                    &format!("{session_name}:{window_name}"),
                ])
                .spawn()
                .expect("FAILED TO ATTACH")
                .wait();
            return true;
        } else {
            let _o = process::Command::new("tmux")
                .args(vec![
                    "attach",
                    "-t",
                    &format!("{session_name}:{window_name}"),
                ])
                .spawn()
                .expect("FAILED TO ATTACH")
                .wait();
            return true;
        }
    }

    pub fn send_keys(session_name: &str, window_name: &str, keys: &str) -> bool {
        return Self::run_tmux_command(vec![
            "send-keys",
            "-t",
            &format!("{session_name}:{window_name}"),
            keys,
            "C-m",
        ]);
    }

    pub fn watch_command(
        session_name: &str,
        window_name: &str,
        hidden: bool,
        horizontal: bool,
        command: &str,
    ) -> bool {
        let dir_arg: &str;
        if horizontal {
            dir_arg = "-h";
        } else {
            dir_arg = "-v";
        }

        if hidden {
            return Self::run_tmux_command(vec![
                "split-window",
                "-d",
                &dir_arg,
                "-t",
                &format!("{session_name}:{window_name}"),
                "-Z",
                "watch",
                &command,
            ]);
        } else {
            return Self::run_tmux_command(vec![
                "split-window",
                "-d",
                &dir_arg,
                "-t",
                &format!("{session_name}:{window_name}"),
                "watch",
                &command,
            ]);
        }

        return false;
    }
}
