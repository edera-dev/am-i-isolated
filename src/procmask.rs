use std::fs::File;
use anyhow::Result;

use crate::{Test, TestResult};

pub struct ProcMaskTest {}

#[derive(Default)]
pub struct ProcMaskResult {
    pub masked: bool,
}

impl Test for ProcMaskTest {
    fn name(&self) -> String {
        "whether access to /proc and /sys is masked".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = ProcMaskResult{
            masked: true,
        };

        if let Ok(_f) = File::create("/proc/sysrq-trigger") {
            result.masked = false;
        }

        Ok(Box::new(result))
    }
}

impl TestResult for ProcMaskResult {
    fn success(&self) -> bool {
        self.masked
    }

    fn explain(&self) {
        if self.masked {
            println!("  + Access to /proc and /sys appears to be masked.");
            return;
        }

        println!("  - Access to /proc and /sys appears to NOT be masked.");
        println!("    This container environment can possibly be used to restart the host, as well as");
        println!("    discover more about system configuration.");
    }

    fn as_string(&self) -> String {
        if self.masked {
            return "masked".to_string();
        }

        "not masked".to_string()
    }
}
