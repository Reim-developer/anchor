use crate::cli::{Cli, Commands};
use crate::cmds::cat::cat_command;
use clap::Parser;

pub fn handles_commands() {
    let args = Cli::parse();

    match args.commands {
        Commands::Cat { file_path } => {
            cat_command(&file_path);
        }
    }
}
