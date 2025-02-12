use crate::utils::file::file_exist::file_is_exist;
use crate::utils::file::is_dir::is_directory;
use crate::utils::time::time_calc::cal_time;
use memmap2::Mmap;
use std::error::Error;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;

fn mmap_read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let file_target = File::open(file_path)
        .map_err(|error| Box::<dyn Error>::from(format!("Error when opening file: {}", error)))?;

    let mmap = unsafe {
        Mmap::map(&file_target).map_err(|error| {
            Box::<dyn Error>::from(format!("Error when memory-mapping file: {}", error))
        })?
    };
    Ok(String::from_utf8_lossy(&mmap).to_string())
}

pub fn cat_command(file_path: &str) {
    if is_directory(file_path) || !file_is_exist(file_path) {
        return;
    }
    cal_time(|| match mmap_read_file(file_path) {
        Ok(result) => {
            sleep(Duration::from_millis(500));
            print!("{}", result);
            String::new()
        }

        Err(err) => {
            eprintln!("Error when reading file {}", err);
            String::new()
        }
    });
}
