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
        "for docker socket".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = DockerResult { allowed: false };

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

    fn explain(&self) {
        if !self.allowed {
            println!("  + No usable docker socket was found.");
            return;
        }

        println!("  - Why: `docker run --privileged` can be used to compromise the system through");
        println!("         the use of privileged containers.");
        println!("  - Suggestion: Don't share the docker socket with untrusted containers.");
    }

    fn as_string(&self) -> String {
        if self.allowed {
            return "usable".to_string();
        }

        "not usable".to_string()
    }
}
