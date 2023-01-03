use crate::config::SSH;
use std::{collections::HashMap};

pub fn get_servers(ssh: &HashMap<String, SSH>) -> Vec<String> {
    let mut server_list: Vec<String> = Vec::new();

    for (alias, details) in &*ssh {
        server_list.push(format!("{alias}: {0}@{1}", details.username, details.host));
    }

    return server_list;
}
