use anyhow::Result;
use std::{env, fs};

use crate::{Test, TestCategory, TestResult};

pub struct OCITest {}

#[derive(Default)]
pub struct OCIResult {
    pub present: bool,
    pub runtime: String,
}

impl Test for OCITest {
    fn name(&self) -> String {
        "OCI runtime".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = OCIResult {
            present: false,
            runtime: "".to_string(),
        };

        match env::var("container") {
            Ok(val) => {
                result.present = true;
                result.runtime = val
            }
            Err(_) => {
                result.present = false;
            }
        }

        if let Ok(exists) = fs::exists("/.dockerenv") {
            if exists {
                result.present = true;
                result.runtime = "docker".to_string();
            }
        }

        Ok(Box::new(result))
    }

    fn category(&self) -> TestCategory {
        TestCategory::Low
    }
}

impl TestResult for OCIResult {
    fn success(&self) -> bool {
        self.present
    }

    fn explain(&self) -> String {
        if !self.present {
            return "no OCI runtime environment found".to_string();
        }

        format!("{} container runtime found", self.runtime).to_string()
    }

    fn as_string(&self) -> String {
        if self.present {
            return self.runtime.clone();
        }

        "not present".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2000".to_string()
    }
}
