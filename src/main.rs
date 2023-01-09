mod config;
mod dirs;
mod fuzzy_finder;
mod notes;
mod ssh;
mod tasks;
mod tmux;

use clap::{Args, Parser, Subcommand};

use dirs::DirsCommand;
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

    /// Launch Task Manager
    Tasks {},

    /// Launch Notes
    Notes {},

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
            TasksCommand::healthcheck(true);
            println!("\nPlease install all missing requirements from the above.");
        }
    }
}
