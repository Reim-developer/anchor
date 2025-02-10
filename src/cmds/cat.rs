use crate::utils::file_exist::file_is_exist;
use crate::utils::is_dir::is_directory;
use memmap2::Mmap;
use std::{
    fs::File,
    io::{self, Write},
};

fn mmap_read_file(file_path: &str) -> Result<(), io::Error> {
    let file_target = File::open(file_path).map_err(|error| {
        eprintln!("Error when opening file: {}", error);
        error
    })?;
    let mmap = unsafe {
        Mmap::map(&file_target).map_err(|error| {
            eprintln!("Error when memory-mapping file: {}", error);
            error
        })
    }?;
    io::stdout().lock().write_all(&mmap)?;
    Ok(())
}

pub fn cat_command(file_path: &str) {
    if is_directory(file_path) || !file_is_exist(file_path) {
        return;
    }
    if let Err(error) = mmap_read_file(file_path) {
        eprintln!("Found error: {}", error);
    }
}
