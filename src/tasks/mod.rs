pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::{Task, NewTask};

pub fn add_task(conn: &mut SqliteConnection, task_name: &str) -> usize {
    use schema::tasks;

    let new_task = NewTask { task_name };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn show_tasks(conn: &mut SqliteConnection) -> () {
    use self::schema::tasks::dsl::*;

    let results = tasks
        .load::<Task>(conn)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}: {}", task.id, task.task_name);
    }
}
