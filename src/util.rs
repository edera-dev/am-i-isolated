use std::fs::read_to_string;
use std::path::Path;

pub fn read_file_as_lines<P: AsRef<Path>>(p: P) -> Result<Vec<String>, ()> {
    if let Ok(file) = read_to_string(p) {
        return Ok(file
            .lines()
            .map(String::from)
            .collect())
    }

    Err(())
}
