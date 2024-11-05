use anyhow::Result;

use crate::{util::read_file_as_space_separated_lines, Test, TestResult};

pub struct UptimeTest {}

#[derive(Default)]
pub struct UptimeResult {
    pub visible: bool,
    pub uptime: u64,
}

impl Test for UptimeTest {
    fn name(&self) -> String {
        "for system uptime leakage".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = UptimeResult {
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
}

impl TestResult for UptimeResult {
    fn success(&self) -> bool {
        !self.visible
    }

    fn explain(&self) {
        if !self.visible {
            println!("  + System uptime is not visible.");
            return;
        }

        if self.uptime <= 10 {
            println!(
                "  + System uptime is available, but likely represents the container lifetime."
            );
            println!("  + This mode is safer as uptime is available on the system but does not leak information.");
            return;
        }

        println!("  - Why: System uptime and similar channels of information can provide real information about the container host.");
        println!("         For example, it could leak whether or not the system has been patched quickly for new vulnerabilities.");
        println!("  - Suggestion: Utilizing container isolation such as gVisor, Kata Containers, or Edera Protect can prevent access to some types of information.");
        println!("                Kata Containers and Edera Protect eliminate the full risk of a shared kernel, whereas gVisor limits a majority of the risks.");
    }

    fn as_string(&self) -> String {
        if !self.visible {
            return "not visible".to_string();
        }

        if self.uptime <= 10 {
            return "visible, likely container uptime".to_string();
        }

        "visible, likely system uptime".to_string()
    }
}
