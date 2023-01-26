pub mod db;
pub mod tasks;

use clap::{Parser, Subcommand};
use tasks::{add_task, show_tasks};
use db::establish_connection;

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
    Remove {},
}

#[derive(Subcommand)]
enum Profiles {
    Show {},
    Add {},
    Edit {},
    Remove {},
}

fn main() {
    let cli = Cli::parse();
    let conn = &mut establish_connection();

    match &cli.command {
        Commands::Task { command } => match &command {
            Tasks::Show {} => show_tasks(),
            Tasks::Add { task_message } => _ = add_task(conn, task_message),
            Tasks::Edit {} => {}
            Tasks::Remove {} => {}
        },

        Commands::Profile { command } => match &command {
            Profiles::Show {} => {}
            Profiles::Add {} => {}
            Profiles::Edit {} => {}
            Profiles::Remove {} => {}
        },
    }
}
