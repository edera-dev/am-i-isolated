use anyhow::Result;
use std::fs::File;

use crate::{Test, TestCategory, TestResult};

pub struct ProcMaskTest {}

#[derive(Default)]
pub struct ProcMaskResult {
    pub masked: bool,
}

impl Test for ProcMaskTest {
    fn name(&self) -> String {
        "/proc and /sys masking".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = ProcMaskResult { masked: true };

        if let Ok(_f) = File::create("/proc/sysrq-trigger") {
            result.masked = false;
        }

        Ok(Box::new(result))
    }

    fn category(&self) -> TestCategory {
        TestCategory::High
    }
}

impl TestResult for ProcMaskResult {
    fn success(&self) -> bool {
        self.masked
    }

    fn explain(&self) -> String {
        if self.masked {
            return "access to /proc and /sys is masked".to_string();
        }

        "access to /proc and /sys is not masked, can be used to restart the host and discover system configuration data".to_string()
    }

    fn as_string(&self) -> String {
        if self.masked {
            return "masked".to_string();
        }

        "not masked".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2230".to_string()
    }
}
