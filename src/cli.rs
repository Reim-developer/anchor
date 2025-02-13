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
        #[arg(short, long = "file", help = "Show all content in file")]
        file_path: String,
    },

    #[command(name = "hash")]
    Hash {
        #[arg(short, long = "file", help = "Show hash of file")]
        file_path: String,
        #[arg(
            short,
            long = "debug",
            help = "Enable debug file",
            default_value = "false"
        )]
        debug: bool,
    },
    #[command(name = "fmt")]
    Fmt {
        #[arg(short = 'f', long = "file", help = "Format file")]
        file_path: String,
    },
}
