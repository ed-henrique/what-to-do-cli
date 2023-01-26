pub mod models;
pub mod schema;

use what_to_do_cli::*;
use diesel::prelude::*;
use crate::models::NewTask;

pub fn add_task(conn: &mut SqliteConnection, task_name: &str) -> usize {
    use crate::schema::tasks;

    let new_task = NewTask { task_name };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn show_tasks() -> () {
    use self::schema::tasks::dsl::*;

    let connection = &mut establish_connection();
    let results = tasks
        .limit(5)
        .load::<Task>(connection)
        .expect("Error loading tasks");

    println!("Displaying {} tasks", results.len());
    for task in results {
        println!("{}: {}", task.id, task.task_name);
        println!("----------\n");
    }
}
