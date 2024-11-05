use libc;
use anyhow::Result;

use crate::{Test, TestResult};

pub struct RootTest {}

#[derive(Default)]
pub struct RootResult {
    pub uid: libc::uid_t,
}

impl Test for RootTest {
    fn name(&self) -> String {
        "running as root".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let result = RootResult{
            uid: unsafe { libc::getuid() },
        };

        Ok(Box::new(result))
    }
}

impl TestResult for RootResult {
    fn success(&self) -> bool {
        self.uid != 0
    }

    fn explain(&self) -> String {
        if self.uid != 0 {
            return "".to_string();
        }

        "workload is running as root".to_string()
    }

    fn as_string(&self) -> String {
        if self.success() {
            return "no".to_string();
        }

        "yes".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2250".to_string()
    }
}
