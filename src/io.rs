use std::io::Error;
use std::path::{self, Path};
use std::fs::{File, OpenOptions};


pub struct IO {

}

impl IO {
    pub fn create_file(path: &Path) -> Result<File, Error> {
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path);

        match file {
            Ok(file) => Ok(file),
            Err(e) => {
                println!("File {} already exists! Please try a different path", path.display());
                Err(e)
            }
        }
    }
}