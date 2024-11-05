use anyhow::Result;
use std::{env, fs};

use crate::{Test, TestResult};

pub struct OCITest {}

#[derive(Default)]
pub struct OCIResult {
    pub present: bool,
    pub runtime: String,
}

impl Test for OCITest {
    fn name(&self) -> String {
        "for OCI runtime presence".to_string()
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
}

impl TestResult for OCIResult {
    fn success(&self) -> bool {
        self.present
    }

    fn explain(&self) {
        if self.present {
            println!("  + OCI container is: {:?}", self.runtime);
        }
    }

    fn as_string(&self) -> String {
        if self.present {
            return self.runtime.clone();
        }

        "not present".to_string()
    }
}
