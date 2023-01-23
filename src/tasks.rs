use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn add_task(task_message: &String) -> () {
    // This code will be heavily changed when SQLite starts to be used, but for now it does what it's supposed to do

    let file_name = "tmp_file.txt";
    let directory_path = "../tmp/";
    let full_path = &format!("{}{}", directory_path, file_name);

    if !Path::new(directory_path).exists() {
        fs::create_dir(directory_path).ok();
    }

    if !Path::new(full_path).exists() {
        File::create(full_path).ok();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(full_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", task_message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
