use anyhow::Result;

use crate::{Test, TestResult};
use crate::util::read_file_as_lines;

pub struct YamaTest {}

#[derive(Default)]
pub struct YamaResult {
    pub present: bool,
}

impl Test for YamaTest {
    fn name(&self) -> String {
        "Yama LSM".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = YamaResult{
            present: false,
        };

        if let Ok(lines) = read_file_as_lines("/proc/kallsyms") {
            for line in lines {
                if line.contains("yama_lsmid") {
                    result.present = true;
                }
            }
        }

        Ok(Box::new(result))
    }
}

impl TestResult for YamaResult {
    fn success(&self) -> bool {
        self.present
    }

    fn explain(&self) -> String {
        if self.present {
            return "".to_string();
        }

        "Yama LSM not present, enabling it would prevent several ptrace-based escape attacks".to_string()
    }

    fn as_string(&self) -> String {
        if self.present {
            return "present".to_string();
        }

        "not present".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2270".to_string()
    }
}
