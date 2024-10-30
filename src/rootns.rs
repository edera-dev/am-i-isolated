use std::fs;
use anyhow::Result;

use crate::{Test, TestResult};

pub struct RootNSTest {}

#[derive(Default)]
pub struct RootNSResult {
    pub pid_nsid: u64,
    pub net_nsid: u64,
    pub ipc_nsid: u64,
}

fn resolve_nsid(ns: &str) -> u64 {
    if let Ok(link) = fs::read_link("/proc/self/ns/".to_owned() + ns) {
        if let Ok(rawlink) = link.into_os_string().into_string() {
            let parts: Vec<_> = rawlink.split(':').collect();
            let raw_nsid = &parts[1];
            let cooked_nsid = &raw_nsid[1..raw_nsid.len() - 1];
            
            if let Ok(parsed_nsid) = u64::from_str_radix(cooked_nsid, 10) {
                return parsed_nsid;
            }
        }
    }

    0
}

impl Test for RootNSTest {
    fn name(&self) -> String {
        "whether any host namespaces are exposed".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let result = RootNSResult{
            pid_nsid: resolve_nsid("pid"),
            net_nsid: resolve_nsid("net"),
            ipc_nsid: resolve_nsid("ipc"),
        };

        Ok(Box::new(result))
    }
}

impl TestResult for RootNSResult {
    fn success(&self) -> bool {
        self.pid_nsid > 0xf0000001 && self.net_nsid > 0xf0000001 && self.ipc_nsid > 0xf0000001
    }

    fn explain(&self) {
        if self.success() {
            println!("  + All checked namespace IDs were dynamically allocated.");
            return;
        }

        if self.pid_nsid < 0xf0000002 {
            println!("  - Host PID namespace is present.");
        }

        if self.net_nsid < 0xf0000002 {
            println!("  - Host networking namespace is present.");
        }

        if self.ipc_nsid < 0xf0000002 {
            println!("  - Host IPC namespace is present.");
        }
    }

    fn as_string(&self) -> String {
        if self.success() {
            return "no".to_string();
        }

        "yes".to_string()
    }
}
