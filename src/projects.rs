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
