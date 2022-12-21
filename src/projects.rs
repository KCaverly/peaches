use skim::prelude::*;
use std::io;
use std::io::Cursor;
use std::process::exit;
use walkdir::{DirEntry, WalkDir};

fn folders(path: &str, max_depth: u8) -> Result<Vec<DirEntry>, io::Error> {
    Ok(WalkDir::new(path)
        .min_depth(max_depth.into())
        .max_depth(max_depth.into())
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|r| r.unwrap())
        .filter(|x| x.path().is_dir())
        .collect::<Vec<DirEntry>>())
}

struct ParentDirectory {
    path: String,
    include_hidden: bool,
    max_depth: u8,
}

pub fn get_files() -> Vec<String> {
    let directory_list = vec![
        ParentDirectory {
            path: "/home/kcaverly/personal".to_string(),
            include_hidden: false,
            max_depth: 1,
        },
        ParentDirectory {
            path: "/home/kcaverly/.dotfiles".to_string(),
            include_hidden: true,
            max_depth: 3,
        },
        ParentDirectory {
            path: "/home/kcaverly/work".to_string(),
            include_hidden: true,
            max_depth: 2,
        },
    ];

    let mut file_list: Vec<String> = Vec::new();

    for directory in directory_list.iter() {
        let dir_files: Vec<String> = folders(&directory.path, directory.max_depth)
            .unwrap()
            .iter()
            .map(|x| x.path().to_str().unwrap().to_string())
            .filter(|x| {
                if directory.include_hidden {
                    true
                } else {
                    !x.starts_with(".")
                }
            })
            .map(|x| x.to_string())
            .filter(|x| !x.contains(".git"))
            .filter(|x| !x.contains("bin"))
            .collect();

        for file in dir_files.iter() {
            if file.to_string() != directory.path {
                file_list.push(file.to_string());
            }
        }
    }

    return file_list;
}

pub fn search_options(search_options: Vec<String>) -> String {
    let search_string: String = search_options.join("\n");
    let options = SkimOptionsBuilder::default()
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(search_string));
    let selected_item = Skim::run_with(&options, Some(items))
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    // If no item selected, exit silently.
    if selected_item.len() == 0 {
        exit(0)
    }

    let option = selected_item
        .first()
        .expect("No Option Selected!")
        .output()
        .to_string();

    return option;
}
