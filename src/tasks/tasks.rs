use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn add_task(conn: &mut SqliteConnection, task_name: &str) -> usize {
    use crate::schema::tasks;

    let new_task = NewTask { task_name };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new post")
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
