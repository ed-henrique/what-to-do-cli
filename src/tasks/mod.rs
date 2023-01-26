pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::{NewTask, Task};

pub fn add_task(conn: &mut SqliteConnection, task_name: &str) -> () {
    use schema::tasks;

    let new_task = NewTask { task_name };

    diesel::insert_into(tasks::table)
        .values(&new_task)
        .execute(conn)
        .expect("Error saving new post");

    println!("Added task: {}!", task_name);
}

pub fn delete_task(conn: &mut SqliteConnection, task_id: &i32) -> () {
    use schema::tasks::dsl::*;

    diesel::delete(tasks.filter(id.eq(task_id)))
        .execute(conn)
        .expect("Error deleting task {task_id}");

    println!("Deleted task: {}!", task_id);
}

pub fn show_tasks(conn: &mut SqliteConnection) -> () {
    use schema::tasks::dsl::*;

    let results = tasks.load::<Task>(conn).expect("Error loading tasks");

    println!("Displaying tasks:\n");
    
    for task in results {
        println!("{}: {}", task.id, task.task_name);
    }
}
