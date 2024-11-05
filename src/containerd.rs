use anyhow::Result;
use std::os::unix::net::UnixStream;

const CONTAINERD_SOCKET_LOCATION: &str = "/run/containerd/containerd.sock";

use crate::{Test, TestResult};

pub struct ContainerDTest {}

#[derive(Default)]
pub struct ContainerDResult {
    pub allowed: bool,
}

impl Test for ContainerDTest {
    fn name(&self) -> String {
        "containerd socket".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = ContainerDResult{
            allowed: false,
        };

        let usable = UnixStream::connect(CONTAINERD_SOCKET_LOCATION).map_or(false, |_| true);
        if usable {
            result.allowed = true;
        }

        Ok(Box::new(result))
    }
}

impl TestResult for ContainerDResult {
    fn success(&self) -> bool {
        !self.allowed
    }

    fn explain(&self) -> String {
        if !self.success() {
            return "".to_string();
        }

        return "ContainerD socket found, `nerdctl run --privileged` can be used to escape".to_string();
    }

    fn as_string(&self) -> String {
        if self.allowed {
            return "usable".to_string();
        }

        "not usable".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2210".to_string()
    }
}
