use anyhow::Result;
use libc;

use crate::{Test, TestResult};

pub struct RootTest {}

#[derive(Default)]
pub struct RootResult {
    pub uid: libc::uid_t,
}

impl Test for RootTest {
    fn name(&self) -> String {
        "whether the workload is running as root".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let result = RootResult {
            uid: unsafe { libc::getuid() },
        };

        Ok(Box::new(result))
    }
}

impl TestResult for RootResult {
    fn success(&self) -> bool {
        self.uid != 0
    }

    fn explain(&self) {
        if self.uid != 0 {
            println!("  + The workload is not running as root.");
            return;
        }

        println!("  - The workload is running as root.");
        println!("    Even with user namespaces, it is a bad practice to run workloads as root.");
    }

    fn as_string(&self) -> String {
        if self.success() {
            return "no".to_string();
        }

        "yes".to_string()
    }
}
