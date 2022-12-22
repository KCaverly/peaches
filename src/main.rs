mod config;
mod projects;
mod tmux;

use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use std::str;

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run projects fuzzy finder
    Projects {},
    /// Upgrade peaches to latest version available on github
    Upgrade {},
}

fn run_upgrade() {
    println!("Getting new install script from peaches repository.\n");

    let get_script = Command::new("wget")
        .args(vec![
            "https://raw.githubusercontent.com/KCaverly/peaches/main/install.sh",
            "-O",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    println!("Installing new version of peaches\n");

    let install_script = Command::new("sh")
        .stdin(Stdio::from(get_script.stdout.unwrap()))
        .spawn()
        .unwrap();

    let output = install_script.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    println!("{}", result);
}

fn run_projects() {
    // Get Files and Search
    let files = projects::get_files();
    let selected = projects::search_options(files);
    let name = selected.split("/").last().unwrap();

    // Launch Project
    tmux::TMUX::create_window("kc", name);
    tmux::TMUX::send_keys("kc", name, &format!("cd {selected} && clear"));
    tmux::TMUX::attach_or_select_window("kc", name);
}

fn main() {
    config::load_config();

    let value = Value::parse();

    match &value.commands {
        Commands::Projects {} => {
            run_projects();
        }
        Commands::Upgrade {} => {
            run_upgrade();
        }
    }
}
