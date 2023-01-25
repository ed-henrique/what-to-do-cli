use self::models::*;
use diesel::prelude::*;
use what_to_do_cli::*;

fn main() {
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
