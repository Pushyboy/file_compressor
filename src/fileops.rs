use std::io::Error;
use std::path::Path;
use std::fs::{File, OpenOptions};

/// Creates a file with read and write access
/// 
/// Returns [`Result<File, Error>`] if the file already exists
pub fn create_file(path: &Path) -> Result<File, Error> {
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path);

    match file {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("File {} already exists! Please try a different path", path.display());
            Err(e)
        }
    }
}