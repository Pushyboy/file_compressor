#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use super::*;
    use crate::{lzss::LZSS, fileops};

    #[test]
    fn test() {
        let mut input = File::open("C:/Users/pleas/Downloads/VSCode/file_compressor/green.txt").unwrap();
        let path = Path::new("C:/Users/pleas/Downloads/VSCode/file_compressor/out.txt");
        let mut output = fileops::create_file(path).unwrap();
        LZSS::write_output(&mut input, &mut output);
    }

    #[test]
    fn test_1() {
        let mut input = File::open("C:/Users/pleas/Downloads/VSCode/file_compressor/out.txt").unwrap();
        let path = Path::new("C:/Users/pleas/Downloads/VSCode/file_compressor/out-decoded.txt");
        let mut output = fileops::create_file(path).unwrap();
        LZSS::read_input(&mut input, &mut output);
    }
}