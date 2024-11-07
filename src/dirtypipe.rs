use anyhow::Result;

use crate::{util::kernel_release_info, Test, TestCategory, TestResult};

pub struct DirtyPipeTest {}

#[derive(Default)]
pub struct DirtyPipeResult {
    pub vulnerable: bool,
    pub kernel_version: String,
}

impl Test for DirtyPipeTest {
    fn name(&self) -> String {
        "dirty pipe (CVE-2022-0847) vulnerability".to_string()
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

    fn category(&self) -> TestCategory {
        TestCategory::High
    }
}

impl TestResult for DirtyPipeResult {
    fn success(&self) -> bool {
        !self.vulnerable
    }

    fn explain(&self) -> String {
        if !self.vulnerable {
            return format!(
                "kernel '{}' not vulnerable to dirty pipe (CVE-2022-0847)",
                self.kernel_version
            );
        }

        format!(
            "kernel '{}' vulnerable to dirty pipe (CVE-2022-0847), upgrade to kernel >= 5.17 (or latest 5.15 LTS)",
            self.kernel_version
        )
    }

    fn as_string(&self) -> String {
        if self.vulnerable {
            return "vulnerable".to_string();
        }

        "not vulnerable".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2300".to_string()
    }
}
