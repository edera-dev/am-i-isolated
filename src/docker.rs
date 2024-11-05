use anyhow::Result;
use std::os::unix::net::UnixStream;

const DOCKER_SOCKET_LOCATION: &str = "/var/run/docker.sock";

use crate::{Test, TestResult};

pub struct DockerTest {}

#[derive(Default)]
pub struct DockerResult {
    pub allowed: bool,
}

impl Test for DockerTest {
    fn name(&self) -> String {
        "docker socket".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = DockerResult{
            allowed: false,
        };

        let usable = UnixStream::connect(DOCKER_SOCKET_LOCATION).map_or(false, |_| true);
        if usable {
            result.allowed = true;
        }

        Ok(Box::new(result))
    }
}

impl TestResult for DockerResult {
    fn success(&self) -> bool {
        !self.allowed
    }

    fn explain(&self) -> String {
        if self.success() {
            return "".to_string();
        }

        "Docker socket found, `docker run --privileged` can be used to escape".to_string()
    }

    fn as_string(&self) -> String {
        if self.allowed {
            return "usable".to_string();
        }

        "not usable".to_string()
    }

    fn fault_code(&self) -> String {
        "AII2211".to_string()
    }
}
