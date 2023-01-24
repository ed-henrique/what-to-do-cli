use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

// This code will be heavily changed when SQLite starts to be used, but for now it does what it's supposed to do

static DIRECTORY_PATH: &str = "../tmp/";
static FULL_PATH: &str = "../tmp/tmp_file.txt";

pub fn add_task(task_message: &String) -> () {
    if !Path::new(DIRECTORY_PATH).exists() {
        fs::create_dir(DIRECTORY_PATH).ok();
    }

    if !Path::new(FULL_PATH).exists() {
        File::create(FULL_PATH).ok();
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FULL_PATH)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", task_message) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

pub fn show_tasks() -> () {

}