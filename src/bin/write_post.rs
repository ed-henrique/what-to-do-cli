use std::io::stdin;
use what_to_do_cli::*;

fn main() {
    let connection = &mut establish_connection();

    let mut task_name = String::new();

    println!("What would you like your task to be?");
    stdin().read_line(&mut task_name).unwrap();
    let task_name = task_name.trim_end();

    let _ = create_task(connection, task_name);
    println!("\nSaved draft {}", task_name);
}
