mod config;
mod dirs;
mod tmux;
mod fuzzy_finder;
mod notes;
mod tasks;
mod ssh;

use clap::{Args, Parser, Subcommand};

use dirs::DirsCommand;
use ssh::SSHCommand;
use tasks::TasksCommand;
use notes::NotesCommand;

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
}

#[derive(Debug, Args)]
#[command(arg_required_else_help=true)]
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

        Commands::Dirs { } => {
            let cfg = config::load_config();
            DirsCommand::run(&cfg);
        }

        Commands::SSH { } => {
            let cfg = config::load_config();
            SSHCommand::run(&cfg);
        }

        Commands::Tasks {} => {
            let cfg = config::load_config();
            TasksCommand::run(&cfg);
        }

        Commands::Notes {} => {
            let cfg = config::load_config();
            NotesCommand::run(&cfg);
        }

        Commands::Config(config) => {
            match config.command.unwrap() {

                ConfigCommands::Init {} => {}

                ConfigCommands::Encrypt { password } => {
                    println!("Password {:?}", password);
                }
            }
        }
    }

    // Continued program logic goes here...
}
