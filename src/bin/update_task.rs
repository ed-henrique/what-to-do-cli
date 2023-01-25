use diesel::prelude::*;
use std::env::args;
use what_to_do_cli::*;

fn main() {
    use self::schema::tasks::dsl::{task_name, tasks};

    let id = args()
        .nth(1)
        .expect("update_task requires a task id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let _ = diesel::update(tasks.find(id))
        .set(task_name.eq("oi"))
        .execute(connection)
        .unwrap();

    let task: models::Task = tasks
        .find(id)
        .first(connection)
        .unwrap_or_else(|_| panic!("Unable to find task {}", id));

    println!("Updated task {}", task.task_name);
}
