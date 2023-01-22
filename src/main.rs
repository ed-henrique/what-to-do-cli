use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Task {
        #[command(subcommand)]
        command: Option<Tasks>,
    },
    Profile {
        #[command(subcommand)]
        command: Option<Profiles>,
    },
}

#[derive(Subcommand)]
enum Tasks {
    Show {},
    Add {},
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
    let _cli = Cli::parse();
}
