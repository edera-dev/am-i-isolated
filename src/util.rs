use std::collections::HashMap;
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

pub fn read_file_as_tuples<P: AsRef<Path>>(p: P) -> Result<HashMap<String, String>, ()> {
    if let Ok(lines) = read_file_as_lines(p) {
        let mut map = HashMap::new();

        for line in lines {
            let fields: Vec<_> = line.split(':').collect();
            let key = String::from(fields[0]);
            let value = String::from(fields[1]);

            map.insert(key, String::from(value.trim()));
        }

        return Ok(map)
    }

    Err(())
}
