mod config;
mod dirs;
mod docker;
mod fuzzy_finder;
mod notes;
mod ssh;
mod tasks;
mod tmux;

use clap::{Parser, Subcommand};
use dirs::DirsCommand;
use docker::DockerCommand;
use ssh::SSHCommand;
use notes::NotesCommand;
use std::process::{Command, Stdio};
use std::str;
use tasks::TasksCommand;

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run projects fuzzy finder
    Dirs {},
    /// Run ssh server launcher
    SSH {},
    /// Run tasks launcher
    Tasks {},
    /// Run notes launcher
    Notes {},
    /// Upgrade peaches to latest version available on github
    Upgrade {},
    /// Initialize config file
    Config {},
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

fn run_encrypt(raw_string: &str) {
    println!("Raw: {}", raw_string);
    println!("Encrypted: {}", config::Config::encrypt(raw_string));
}

fn main() {
    let value = Value::parse();

    match &value.commands {
        Commands::Dirs {} => {
            let cfg: config::Config = config::load_config();
            DirsCommand::run(&cfg);
        }

        Commands::SSH {} => {
            let cfg: config::Config = config::load_config();
            SSHCommand::run(&cfg);
        }

        Commands::Tasks {} => {
            let cfg: config::Config = config::load_config();
            TasksCommand::run(&cfg)
        }

        Commands::Notes {} => {
            let cfg: config::Config = config::load_config();
            NotesCommand::run(&cfg)
        }

        Commands::Upgrade {} => {
            run_upgrade();
        }

        Commands::Config {} => {
            run_config();
        }

        Commands::Docker {} => DockerCommand::run(),

        // TODO: Move this test_password to subcommand argument
        Commands::Encrypt {} => run_encrypt("test_password"),
    }
}
