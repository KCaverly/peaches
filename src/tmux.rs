
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
}

