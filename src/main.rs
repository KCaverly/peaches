mod projects;
mod tmux;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run projects fuzzy finder
    Projects {},
}

fn run_projects() {
    // Get Files and Search
    let files = projects::get_files();
    let selected = projects::search_options(files);
    let name = selected.split("/").last().unwrap();

    // Launch Project
    tmux::TMUX::create_window("kc", name);
    tmux::TMUX::create_session("kc");
    tmux::TMUX::attach_or_select_window("kc", name);
    tmux::TMUX::send_keys("kc", name, &format!("cd {selected} && clear"));
}

fn main() {
    let value = Value::parse();

    match &value.command {
        Commands::Projects {} => {
            run_projects();
        }
    }
}
