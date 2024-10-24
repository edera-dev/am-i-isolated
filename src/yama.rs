use std::fs::read_to_string;
use anyhow::Result;

use crate::{Test, TestResult};

pub struct YamaTest {}

#[derive(Default)]
pub struct YamaResult {
    pub present: bool,
}

fn read_kallsyms() -> Result<Vec<String>, ()> {
    if let Ok(kallsyms) = read_to_string("/proc/kallsyms") {
        return Ok(kallsyms
            .lines()
            .map(String::from)
            .collect())
    }

    Err(())
}

impl Test for YamaTest {
    fn name(&self) -> String {
        "whether the Yama LSM is present".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = YamaResult{
            present: false,
        };

        if let Ok(lines) = read_kallsyms() {
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

    fn explain(&self) {
        if self.present {
            println!("  + Yama LSM is present.");
            return;
        }

        println!("  - Why: Yama LSM prevents several types of ptrace-based container escapes.");
        println!("  - Suggestion: Use a kernel with Yama LSM enabled.");
    }

    fn as_string(&self) -> String {
        if self.present {
            return "present".to_string();
        }

        "not present".to_string()
    }
}
