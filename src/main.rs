pub mod db;
pub mod tasks;

use clap::{Parser, Subcommand};
use db::establish_connection;
use tasks::{add_task, delete_task, show_tasks};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Task {
        #[command(subcommand)]
        command: Tasks,
    },
    Profile {
        #[command(subcommand)]
        command: Profiles,
    },
}

#[derive(Subcommand)]
enum Tasks {
    Show {},
    Add { task_message: String },
    Edit {},
    Delete { task_id: i32 },
}

#[derive(Subcommand)]
enum Profiles {
    Show {},
    Add {},
    Edit {},
    Delete {},
}

fn main() {
    let cli = Cli::parse();
    let conn = &mut establish_connection();

    match &cli.command {
        Commands::Task { command } => match &command {
            Tasks::Show {} => show_tasks(conn),
            Tasks::Add { task_message } => add_task(conn, task_message),
            Tasks::Edit {} => {}
            Tasks::Delete { task_id } => delete_task(conn, task_id),
        },

        Commands::Profile { command } => match &command {
            Profiles::Show {} => {}
            Profiles::Add {} => {}
            Profiles::Edit {} => {}
            Profiles::Delete {} => {}
        },
    }
}
