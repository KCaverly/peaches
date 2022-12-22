use skim::prelude::*;
use std::io::Cursor;
use std::process::exit;
use std::{collections::HashMap, io};
use walkdir::{DirEntry, WalkDir};

use crate::config::Project;

fn folders(path: &str, min_depth: u8, max_depth: u8) -> Result<Vec<DirEntry>, io::Error> {
    Ok(WalkDir::new(path)
        .min_depth(min_depth.into())
        .max_depth(max_depth.into())
        .into_iter()
        .filter(|x| x.is_ok())
        .map(|r| r.unwrap())
        .filter(|x| x.path().is_dir())
        .collect::<Vec<DirEntry>>())
}

pub fn get_files(projects: &HashMap<String, Project>) -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();

    for (_key, value) in &*projects {
        let dir_files: Vec<String> = folders(&value.directory, value.min_depth, value.max_depth)
            .unwrap()
            .iter()
            .map(|x| x.path().to_str().unwrap().to_string())
            .filter(|x| {
                if value.include_hidden {
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
            if file.to_string() != value.directory {
                file_list.push(file.to_string());
            }
        }
    }

    return file_list;
}

pub fn match_to_project<'a>(path: &'a str, projects: &'a HashMap<String, Project>) -> &'a Project {
    for (_key, value) in projects {
        if path.contains(&value.directory) {
            return value;
        }
    }
    panic!("No project found for directory")
}

pub fn search_options(search_options: Vec<String>) -> String {
    let search_string: String = search_options.join("\n");
    let options = SkimOptionsBuilder::default().build().unwrap();

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
