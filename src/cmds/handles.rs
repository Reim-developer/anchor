use crate::cli::{Cli, Commands};
use crate::cmds::cat::cat_command;
use crate::cmds::fmt::json::json_fmt;
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
        Commands::Json { file_path } => {
            json_fmt(&file_path);
        }
    }
}
