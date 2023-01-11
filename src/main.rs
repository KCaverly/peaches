mod config;
mod dirs;
mod docker;
mod fuzzy_finder;
mod notes;
mod ssh;
mod tasks;
mod tmux;

use clap::{Args, Parser, Subcommand};

use dirs::DirsCommand;
use docker::DockerCommand;
use notes::NotesCommand;
use ssh::SSHCommand;
use std::process::exit;
use tasks::TasksCommand;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "peaches")]
#[command(about = "A Smart Switcher for the Terminal", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Launch fuzzy finder for project directories
    #[command(arg_required_else_help = false)]
    Dirs {},

    /// Launch fuzzy finder for ssh servers
    #[command(arg_required_else_help = false)]
    SSH {},

    /// Launch Docker Container fuzzy finder
    Docker {},

    /// Launch Task Manager
    Tasks {},

    /// Launch Notes
    Notes {},

    /// Launch All
    All {},

    /// Manage configuration
    Config(Config),

    /// Healthcheck
    Healthcheck {},
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
struct Config {
    #[command(subcommand)]
    command: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
enum ConfigCommands {
    /// Initialize Generic Config
    Init {},

    /// Encrypt password
    Encrypt { password: Option<String> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Dirs {} => {
            if !DirsCommand::healthcheck(false) {
                exit(0);
            }

            let cfg = config::load_config();
            DirsCommand::run(&cfg);
        }

        Commands::SSH {} => {
            if !SSHCommand::healthcheck(false) {
                exit(0);
            }

            let cfg = config::load_config();
            SSHCommand::run(&cfg);
        }

        Commands::Docker {} => {
            if !DockerCommand::healthcheck(false) {
                exit(0);
            }

            DockerCommand::run();
        }

        Commands::Tasks {} => {
            if !TasksCommand::healthcheck(false) {
                exit(0);
            }

            let cfg = config::load_config();
            TasksCommand::run(&cfg);
        }

        Commands::Notes {} => {
            let cfg = config::load_config();
            NotesCommand::run(&cfg);
        }

        Commands::All {} => {
            let cfg = config::load_config();

            let mut option_list: Vec<String> = Vec::new();

            for option in DirsCommand::get_options(&cfg.directories) {
                option_list.push(format!("DIRS:     {option}"));
            }

            for option in SSHCommand::get_options(&cfg) {
                option_list.push(format!("SSH:      {option}"));
            }

            for option in DockerCommand::get_options() {
                option_list.push(format!("DOCKER:   {option}"));
            }

            for option in TasksCommand::get_options() {
                option_list.push(format!("TASKS:    {option}"));
            }

            for option in NotesCommand::get_options() {
                option_list.push(format!("NOTES:    {option}"));
            }

            let selected = fuzzy_finder::search_options(option_list);

            if selected.contains("DOCKER: ") {
                DockerCommand::post_search_command(&selected.replace("DOCKER:   ", ""));
            }

            if selected.contains("TASKS: ") {
                TasksCommand::run(&cfg);
            }

            if selected.contains("NOTES: ") {
                NotesCommand::run(&cfg);
            }

            if selected.contains("DIRS: ") {
                DirsCommand::post_search_command(&cfg, &selected.replace("DIRS:     ", ""));
            }

            if selected.contains("SSH: ") {
                SSHCommand::post_search_command(&cfg, &selected.replace("SSH:      ", ""));
            }
        }

        Commands::Config(config) => match config.command.unwrap() {
            ConfigCommands::Init {} => {}

            ConfigCommands::Encrypt { password } => {
                println!("Password {:?}", password);
            }
        },

        Commands::Healthcheck {} => {
            println!("Running Healthcheck for peaches");
            DirsCommand::healthcheck(true);
            SSHCommand::healthcheck(true);
            DockerCommand::healthcheck(true);
            TasksCommand::healthcheck(true);
            println!("\nPlease install all missing requirements from the above.");
        }
    }
}
