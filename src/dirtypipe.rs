use anyhow::Result;

use crate::{util::kernel_release_info, Test, TestResult};

pub struct DirtyPipeTest {}

#[derive(Default)]
pub struct DirtyPipeResult {
    pub vulnerable: bool,
    pub kernel_version: String,
}

impl Test for DirtyPipeTest {
    fn name(&self) -> String {
        "for dirty pipe (CVE-2022-0847) vulnerbility".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let (kernel_version, (major, minor, patch)) = kernel_release_info();

        let mut vulnerable = true;
        if major != 5 {
            vulnerable = false;
        } else if !(8..=16).contains(&minor) {
            vulnerable = false;
        } else if minor == 10 {
            if [102, 92].contains(&minor) {
                vulnerable = false;
            }
        } else if minor == 15 && patch == 25 {
            vulnerable = false;
        } else if minor >= 16 && patch >= 11 {
            vulnerable = false;
        }

        Ok(Box::new(DirtyPipeResult {
            vulnerable,
            kernel_version,
        }))
    }
}

impl TestResult for DirtyPipeResult {
    fn success(&self) -> bool {
        !self.vulnerable
    }

    fn explain(&self) {
        if !self.vulnerable {
            println!(
                "  + Kernel version '{}' is not vulnerable to dirty pipe (CVE-2022-0847)",
                self.kernel_version
            );
            return;
        }

        println!("  - Why: Kernel version '{}' is vulnerable to dirty pipe (CVE-2022-0847), which can be used to exploit the container environment and escape to the host.", self.kernel_version);
        println!("  - Suggestion: Upgrade to a recent kernel version that isn't vulnerable to CVE-2022-0847.");
    }

    fn as_string(&self) -> String {
        if self.vulnerable {
            return "vulnerable".to_string();
        }

        "not vulnerable".to_string()
    }
}
