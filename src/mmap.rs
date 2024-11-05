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
        "whether RWX memory mappings are allowed".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = MmapRWXResult{
            allowed: false,
        };

        unsafe {
            let ptr = libc::mmap(std::ptr::null_mut(), 1024768, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, libc::MAP_PRIVATE | libc::MAP_ANON, 0, 0);
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

    fn explain(&self) {
        if !self.allowed {
            println!("  + RWX memory mappings are not allowed.");
            return;
        }

        println!("  - Why: RWX and WX memory mappings can be abused as part of a larger memory");
        println!("         safety attack chain.");
        println!("  - Suggestion: Deploy the OpenPaX kernel patch for memory safety mitigations.");
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
