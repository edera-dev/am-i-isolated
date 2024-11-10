/// Container Virtualization Test
///
/// It is very difficult to detect from within a container whether we are running
/// under virtualization. Am I Isolated relies on heuristics to detect virtualization.
/// This is an incremental process, we expect this test to sometimes fail, particularly
/// in systems like Kata Containers which have no easy way to detect.
///
/// There is a simple fallback mechanism that can give a "maybe" result (represented as a failure.)
/// We read /proc/uptime to attempt to get the system uptime. If the uptime is less than 60 seconds,
/// we assume that the container might be running under virtualization. This makes the assumption
/// that container lifetime is closely related to VM lifetime. In some VM systems, this might not
/// be true. In microVMs, like Edera Protect or firecracker, this should be true if the container
/// is running inside a single Linux kernel boot.
use anyhow::Result;

use crate::{
    util::{is_running_gvisor, read_file_as_space_separated_lines},
    Test, TestCategory, TestResult,
};

const KNOWN_VIRT_RUNTIMES: &[&'static str] = &["edera"];

#[derive(Default, Debug, PartialEq, Eq)]
pub enum VirtualizationEnabled {
    DefinitelyPresent(String),
    MaybePresent,
    #[default]
    NotPresent,
}

#[derive(Default)]
pub struct VirtualizedResult {
    pub enabled: VirtualizationEnabled,
}

pub struct VirtualizedTest;

impl VirtualizedTest {
    pub fn check_definite_runtime_env(&self) -> Option<String> {
        let container_runtime = std::env::var("container").unwrap_or_default();
        KNOWN_VIRT_RUNTIMES
            .iter()
            .find(|runtime| **runtime == container_runtime.as_str())
            .map(|runtime| runtime.to_string())
    }

    pub fn check_definite_gvisor(&self) -> Option<String> {
        if is_running_gvisor() {
            Some("gvisor".to_string())
        } else {
            None
        }
    }

    pub fn check_maybe_present(&self) -> bool {
        let Ok(lines) = read_file_as_space_separated_lines("/proc/uptime") else {
            return false;
        };

        if lines.is_empty() {
            return false;
        }

        let line = &lines[0];
        if line.len() != 2 {
            return false;
        }

        let Ok(uptime) = line[0].parse::<f64>() else {
            return false;
        };

        if uptime < 60.0 {
            return true;
        }
        false
    }
}

impl Test for VirtualizedTest {
    fn name(&self) -> String {
        "container virtualization".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let enabled = if let Some(definite_runtime) = self
            .check_definite_runtime_env()
            .or(self.check_definite_gvisor())
        {
            VirtualizationEnabled::DefinitelyPresent(definite_runtime)
        } else if self.check_maybe_present() {
            VirtualizationEnabled::MaybePresent
        } else {
            VirtualizationEnabled::NotPresent
        };

        Ok(Box::new(VirtualizedResult { enabled }))
    }

    fn category(&self) -> crate::TestCategory {
        TestCategory::High
    }
}

impl TestResult for VirtualizedResult {
    fn success(&self) -> bool {
        matches!(self.enabled, VirtualizationEnabled::DefinitelyPresent(_))
    }

    fn explain(&self) -> String {
        match &self.enabled {
            VirtualizationEnabled::DefinitelyPresent(runtime) => format!("virtualization runtime '{}' found", runtime),
            VirtualizationEnabled::MaybePresent => "signs of virtualization detected, but couldn't definitively determine a virtualization method".to_string(),
            VirtualizationEnabled::NotPresent => "virtualization not detected".to_string(),
        }
    }

    fn fault_code(&self) -> String {
        "AII2280".to_string()
    }

    fn as_string(&self) -> String {
        self.explain()
    }
}
