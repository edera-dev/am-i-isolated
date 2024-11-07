use anyhow::Result;

use crate::{util::read_file_as_space_separated_lines, Test, TestCategory, TestResult};

pub struct VirtualizedTest {}

#[derive(Default)]
pub struct VirtualizedResult {
    pub visible: bool,
    pub uptime: u64,
}

impl Test for VirtualizedTest {
    fn name(&self) -> String {
        "container virtualization".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = VirtualizedResult {
            visible: false,
            uptime: 0,
        };
        if let Ok(lines) = read_file_as_space_separated_lines("/proc/uptime") {
            if !lines.is_empty() {
                let line = &lines[0];
                if line.len() == 2 {
                    if let Ok(uptime) = line[0].parse::<f64>() {
                        result.visible = true;
                        result.uptime = uptime as u64;
                    }
                }
            }
        }
        Ok(Box::new(result))
    }

    fn category(&self) -> crate::TestCategory {
        TestCategory::High
    }
}

impl TestResult for VirtualizedResult {
    fn success(&self) -> bool {
        !self.visible || self.uptime <= 10
    }

    fn explain(&self) -> String {
        if self.success() {
            return "separate kernel used for each container".to_string();
        }
        "without virtualization, the kernel state is shared, opening escape attacks".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2280".to_string()
    }

    fn as_string(&self) -> String {
        if !self.visible {
            return "result not reliable".to_string();
        }

        if self.uptime <= 10 {
            return "virtualization in use".to_string();
        }

        "virtualization not in use".to_string()
    }
}
