use diesel::prelude::*;
use what_to_do_cli::*;
use std::env::args;

fn main() {
    use what_to_do_cli::schema::tasks::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(tasks.filter(task_name.like(pattern)))
        .execute(connection)
        .expect("Error deleting tasks");

    println!("Deleted {} tasks", num_deleted);
}