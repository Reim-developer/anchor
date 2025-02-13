use anchor::utils::styles::colors::LogLevel;
use std::io::{self, Write};
/*
* Test color display
*/
#[test]
fn colors_test_impl() {
    let mut stdout = io::stdout();
    match writeln!(stdout, "ERROR COLOR: {}", LogLevel::Error.fmt()) {
        Ok(_) => {}
        Err(error) => eprintln!("Cannot use stdout with error traceback: {}", error),
    }
    match writeln!(stdout, "INFO COLOR: {}", LogLevel::Info.fmt()) {
        Ok(_) => {}
        Err(error) => eprintln!("Cannot use stdout with error traceback: {}", error),
    }
    match writeln!(stdout, "SUCCESS COLOR: {}", LogLevel::Success.fmt()) {
        Ok(_) => {}
        Err(error) => eprintln!("Cannot use stdout with error traceback: {}", error),
    }
}
