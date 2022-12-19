use skim::prelude::*;
use std::fs;
use std::io;
use std::io::Cursor;
use std::path;

fn folders(dir: &path::Path) -> Result<Vec<path::PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| r.is_dir()) // Filter out non-folders
        .collect())
}

pub fn get_files() -> Vec<String> {
    let directory_list = vec!["/home/kcaverly/personal"];
    let mut file_list: Vec<String> = Vec::new();

    for directory_path in directory_list.iter() {
        let dir_files: Vec<String> = folders(path::Path::new(directory_path))
            .unwrap()
            .iter()
            .map(|x| x.as_path().to_str().unwrap().to_string())
            .filter(|x| !x.starts_with("."))
            .collect();

        for file in dir_files.iter() {
            file_list.push(file.to_string());
        }
    }

    return file_list;
}

pub fn search_options(search_options: Vec<String>) -> String {
    let search_string: String = search_options.join("\n");
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(search_string));
    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    return selected_item.first().expect("No Option Selected!").output().to_string();
}
