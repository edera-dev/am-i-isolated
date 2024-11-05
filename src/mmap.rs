use anyhow::Result;
use libc;

use crate::{Test, TestResult};

pub struct MmapRWXTest {}

#[derive(Default)]
pub struct MmapRWXResult {
    pub allowed: bool,
}

impl Test for MmapRWXTest {
    fn name(&self) -> String {
        "RWX memory mappings".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = MmapRWXResult { allowed: false };

        unsafe {
            let ptr = libc::mmap(
                std::ptr::null_mut(),
                1024768,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
                libc::MAP_PRIVATE | libc::MAP_ANON,
                0,
                0,
            );
            if ptr != libc::MAP_FAILED {
                result.allowed = true;
                libc::munmap(ptr, 1024768);
            }
        }

        Ok(Box::new(result))
    }
}

impl TestResult for MmapRWXResult {
    fn success(&self) -> bool {
        !self.allowed
    }

    fn explain(&self) -> String {
        if !self.allowed {
            return "".to_string();
        }

        "RWX and WX memory mappings can be used as part of a memory safety attack chain".to_string()
    }

    fn as_string(&self) -> String {
        if self.allowed {
            return "allowed".to_string();
        }

        "not allowed".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2220".to_string()
    }
}
