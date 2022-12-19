mod projects;
mod tmux;
// use skim::prelude::*;
// use std::fs;
// use std::io;
// use std::io::Cursor;
// use std::path;

// fn folders(dir: &path::Path) -> Result<Vec<path::PathBuf>, io::Error> {
//     Ok(fs::read_dir(dir)?
//         .into_iter()
//         .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
//         .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
//         .filter(|r| r.is_dir()) // Filter out non-folders
//         .collect())
// }

fn main() {
    // Get Files and Search
    let files = projects::get_files();
    let selected = projects::search_options(files);
    let name = selected.split("/").last().unwrap();

    // Launch Project
    tmux::TMUX::create_session("kc");
    tmux::TMUX::create_window("kc", name);
    tmux::TMUX::attach_or_select_window("kc", name);
    tmux::TMUX::send_keys("kc", name, &format!("cd {selected} && clear"));
}
