use memmap2::Mmap;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

fn is_directory(file_path: &str) -> bool {
    let file_target = Path::new(file_path);

    match file_target.is_dir() {
        true => match file_target.file_name() {
            Some(folder_name) => println!("{}: Is a directory", folder_name.to_string_lossy()),
            None => eprintln!("Can't get direcotory name"),
        },
        false => return false,
    }
    false
}

fn file_is_exist(file_path: &str) -> bool {
    let file_target = Path::new(file_path);

    match file_target.exists() {
        false => {
            eprintln!("{}: File not found", file_target.to_string_lossy());
            false
        }
        true => true,
    }
}

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
