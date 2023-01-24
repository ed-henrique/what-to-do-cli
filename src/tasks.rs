use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

// This code will be heavily changed when SQLite starts to be used, but for now it does what it's supposed to do

static DIRECTORY_PATH: &str = "../tmp/";
static FULL_PATH: &str = "../tmp/tmp_file.txt";

fn check_if_file_exists() -> () {
    if !Path::new(DIRECTORY_PATH).exists() {
        fs::create_dir(DIRECTORY_PATH).ok();
    }

    if !Path::new(FULL_PATH).exists() {
        File::create(FULL_PATH).ok();
    }
}

pub fn add_task(task_message: &String) -> () {
    check_if_file_exists();

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
    check_if_file_exists();

    let mut file = OpenOptions::new().read(true).open(FULL_PATH).unwrap();

    let mut tasks = String::new();
    match file.read_to_string(&mut tasks) {
        Err(why) => panic!("{why}"),
        Ok(_) => print!("{tasks}"),
    }
}
