use std::fs;
use std::path::PathBuf;

pub fn ensure_directory_for_file(path: &PathBuf) {
    if path.is_dir() {
        panic!("the path is not a file path");
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
}
