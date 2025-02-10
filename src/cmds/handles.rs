use crate::cli::{Cli, Commands};
use clap::Parser;

pub fn handles_commands() {
    let args = Cli::parse();

    match args.commands {
        Commands::Cat { file_path } => {
            println!("File Path is: {}", file_path);
        }
    }
}
