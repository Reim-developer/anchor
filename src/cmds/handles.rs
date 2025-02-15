use crate::cli::{Cli, Commands};
use crate::cmds::cat::cat_command;
use crate::cmds::fmt::fmt::fmt;
use crate::cmds::hash::hash_command;
use clap::Parser;

pub fn handles_commands() {
    let args = Cli::parse();

    match args.commands {
        Commands::Cat { file_path } => {
            cat_command(&file_path);
        }
        Commands::Hash { file_path, debug } => {
            hash_command(&file_path, debug);
        }
        Commands::Fmt { file_path } => {
            fmt(&file_path);
        }
    }
}
