use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::ptr::addr_of_mut;

use libc::uname;

pub fn read_file_as_lines<P: AsRef<Path>>(p: P) -> Result<Vec<String>, ()> {
    if let Ok(file) = read_to_string(p) {
        return Ok(file.lines().map(String::from).collect());
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

        return Ok(map);
    }

    Err(())
}

pub fn kernel_release_info() -> (String, (u32, u32, u32)) {
    let release = unsafe {
        let mut uts: libc::utsname = std::mem::zeroed();
        let _ = uname(addr_of_mut!(uts));
        let release_bytes: Vec<u8> = uts
            .release
            .into_iter()
            .map(|x| x as u8)
            .take_while(|x| *x != 0)
            .collect();
        String::from_utf8(release_bytes)
            .ok()
            .unwrap_or_else(|| "0.0.0".to_string())
    };

    let parts: Vec<u32> = release
        .splitn(3, ".")
        .map(|x| x.trim().parse::<u32>().ok().unwrap_or(0))
        .collect();

    (release, (parts[0], parts[1], parts[2]))
}
