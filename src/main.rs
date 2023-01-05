mod config;
mod docker;
mod dotfiles;
mod fuzzy_finder;
mod projects;
mod ssh;
mod tmux;

use clap::{Parser, Subcommand};
use docker::DockerCommand;
use projects::ProjectsCommand;
use ssh::SSHCommand;
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
    /// Run ssh server launcher
    SSH {},
    /// Upgrade peaches to latest version available on github
    Upgrade {},
    /// Initialize config file
    Config {},
    /// Update Dotfiles
    Dotfiles {},
    /// Launch Docker Container Finder
    Docker {},
    /// Helper Function to Encrypt With PEACHES_KEY
    Encrypt {},
}

fn run_config() {
    config::generate_config();
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

fn run_dotfiles(cfg: &config::Config) {
    dotfiles::git_pull_dotfiles(&cfg.dotfiles.location, &cfg.dotfiles.command);
}

fn run_encrypt(raw_string: &str) {
    println!("Raw: {}", raw_string);
    println!("Encrypted: {}", config::Config::encrypt(raw_string));
}

fn main() {
    let value = Value::parse();

    match &value.commands {
        Commands::Projects {} => {
            let cfg: config::Config = config::load_config();
            ProjectsCommand::run(&cfg);
            // run_projects(&cfg);
        }

        Commands::SSH {} => {
            let cfg: config::Config = config::load_config();
            SSHCommand::run(&cfg);
        }

        Commands::Upgrade {} => {
            run_upgrade();
        }

        Commands::Config {} => {
            run_config();
        }

        Commands::Dotfiles {} => {
            let cfg: config::Config = config::load_config();
            run_dotfiles(&cfg);
        }

        Commands::Docker {} => DockerCommand::run(),

        // TODO: Move this test_password to subcommand argument
        Commands::Encrypt {} => run_encrypt("test_password"),
    }
}
