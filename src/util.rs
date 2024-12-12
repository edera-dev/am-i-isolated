use std::fs::read_to_string;
use std::path::Path;
use std::ptr::addr_of_mut;
use std::{collections::HashMap, fs};

use libc::uname;

pub fn read_file_as_lines<P: AsRef<Path>>(p: P) -> Result<Vec<String>, ()> {
    if let Ok(file) = read_to_string(p) {
        return Ok(file.lines().map(String::from).collect());
    }

    Err(())
}

pub fn read_file_as_space_separated_lines<P: AsRef<Path>>(p: P) -> Result<Vec<Vec<String>>, ()> {
    match read_file_as_lines(p) {
        Ok(lines) => Ok(lines
            .into_iter()
            .map(|line| {
                line.split(" ")
                    .map(|part| part.to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()),

        Err(()) => Err(()),
    }
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
        .split(".")
        .map(|x| x.trim().parse::<u32>().ok().unwrap_or(0))
        .collect();

    (release, (parts[0], parts[1], parts[2]))
}

pub fn kernel_cmdline() -> Vec<String> {
    fs::read_to_string("/proc/cmdline")
        .ok()
        .unwrap_or_default()
        .split(" ")
        .map(|part| part.to_string())
        .collect()
}

pub fn is_running_gvisor() -> bool {
    let cmdline = kernel_cmdline();
    if let Some(first) = cmdline.first() {
        first.starts_with("BOOT_IMAGE=/vmlinuz-") && first.ends_with("-gvisor")
    } else {
        false
    }
}
