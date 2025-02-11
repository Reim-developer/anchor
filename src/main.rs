/*
* Define module
*/
mod cli;
mod cmds;
mod core;
mod utils;

use cmds::handles::handles_commands;

fn main() {
    handles_commands();
}
