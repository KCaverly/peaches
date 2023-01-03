use skim::prelude::*;
use std::io::Cursor;
use std::process::exit;

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
