/*
* Define all commands for
* command line interface here
*/
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "cat")]
    Cat {
        #[arg(short, long = "file", help = "The file path you want show content")]
        file_path: String,
    },
}
