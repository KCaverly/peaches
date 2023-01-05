use crate::{
    config::{Config, SSH},
    fuzzy_finder, tmux,
};

pub struct SSHCommand {}

impl SSHCommand {
    fn get_server_alias(selected: &str) -> &str {
        return selected.split(":").next().unwrap();
    }

    fn get_ssh_details<'a>(cfg: &'a Config, selected: &'a str) -> &'a SSH {
        let alias = Self::get_server_alias(selected);
        return &cfg.ssh[alias];
    }

    fn get_options(cfg: &Config) -> Vec<String> {
        let mut server_list: Vec<String> = Vec::new();

        for (alias, details) in &cfg.ssh {
            server_list.push(format!("{alias}: {0}@{1}", details.username, details.host));
        }

        return server_list;
    }

    fn post_search_command(cfg: &Config, selected: &str) {
        let details = Self::get_ssh_details(cfg, selected);

        // Launch TMUX window for SSH Server
        let alias = Self::get_server_alias(selected);
        tmux::TMUX::create_window("ssh", alias);
        tmux::TMUX::send_keys(
            "ssh",
            alias,
            &format!(
                "sshpass -p {0} ssh {1}@{2}",
                &Config::decrypt(&details.password),
                &details.username,
                &details.host
            ),
        );
        tmux::TMUX::attach_or_select_window("ssh", alias);
    }

    pub fn run(cfg: &Config) {
        let options = Self::get_options(cfg);
        let selected = fuzzy_finder::search_options(options);
        Self::post_search_command(&cfg, &selected);
    }
}
