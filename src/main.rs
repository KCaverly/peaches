mod tmux;
mod projects;
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
    println!("In Tmux: {}", tmux::TMUX::in_tmux());

    println!(
        "(TRUE) Session Exists: {}",
        tmux::TMUX::session_exists("kc")
    );
    println!(
        "(FALSE) Session Exists: {}",
        tmux::TMUX::session_exists("test")
    );

    println!(
        "(TRUE) Window Exists: {}",
        tmux::TMUX::window_exists("kc", "peaches")
    );
    println!(
        "(FALSE) Window Exists: {}",
        tmux::TMUX::window_exists("kc", "test")
    );
    println!(
        "(FALSE) Window Exists: {}",
        tmux::TMUX::window_exists("test", "test")
    );

    println!(
        "(TRUE) Create Session: {}",
        tmux::TMUX::create_session("test")
    );
    println!(
        "(TRUE) Session Exists: {}",
        tmux::TMUX::session_exists("test")
    );

    println!(
        "(TRUE) Create Window: {}",
        tmux::TMUX::create_window("test", "test")
    );
    println!(
        "(TRUE) Create Window: {}",
        tmux::TMUX::create_window("test2", "test")
    );

    println!("{}", tmux::TMUX::attach_or_select_window("kc", "gma_cs"));
    println!("{}", tmux::TMUX::attach_or_select_window("test2", "test"));
    println!(
        "{}",
        tmux::TMUX::send_keys("test2", "test", "echo 'hi kyle'")
    );

    // println!("Window Exists: {}", tmux::TMUX::window_exists("kc", "test"));
    // // let project_paths: Vec<String> = folders(path::Path::new("/home/kcaverly/personal"))
    // println!("Create Window: {}", tmux::TMUX::create_window("kc", "test"));
    //     .unwrap()
    //     .iter()
    //     .map(|x| x.as_path().to_str().unwrap().to_string())
    //     .filter(|x| !x.starts_with("."))
    //     .collect();
    //
    // let search_string: String = project_paths.join("\n");
    // let options = SkimOptionsBuilder::default()
    //     .height(Some("50%"))
    //     .multi(true)
    //     .build()
    //     .unwrap();
    //
    // // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // // `SkimItem` was implemented for `AsRef<str>` by default
    // let item_reader = SkimItemReader::default();
    // let items = item_reader.of_bufread(Cursor::new(search_string));
    //
    // // `run_with` would read and show items from the stream
    // let selected_items = Skim::run_with(&options, Some(items))
    //     .map(|out| out.selected_items)
    //     .unwrap_or_else(|| Vec::new());
    //
    // for item in selected_items.iter() {
    //     print!("{}", item.output());
    // }
}
