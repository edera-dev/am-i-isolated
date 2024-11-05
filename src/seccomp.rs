use anyhow::Result;

use crate::{Test, TestResult};
use crate::util::read_file_as_tuples;

pub struct SeccompTest {}

#[derive(Default)]
pub struct SeccompResult {
    pub present: bool,
}

impl Test for SeccompTest {
    fn name(&self) -> String {
        "seccomp profile".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = SeccompResult{
            present: false,
        };

        if let Ok(stat) = read_file_as_tuples("/proc/self/status") {
            if stat["Seccomp"] != "0" {
                result.present = true;
            }
        }

        Ok(Box::new(result))
    }
}

impl TestResult for SeccompResult {
    fn success(&self) -> bool {
        self.present
    }

    fn explain(&self) -> String {
        if self.present {
            return "".to_string();
        }

        "no seccomp profile was found".to_string()
    }

    fn as_string(&self) -> String {
        if self.present {
            return "present".to_string();
        }

        "not present".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2260".to_string()
    }
}
