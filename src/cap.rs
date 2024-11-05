use anyhow::Result;
use std::collections::HashMap;

use crate::util::read_file_as_tuples;
use crate::{Test, TestResult};

pub struct CapTest {}

#[derive(Default)]
pub struct CapResult {
    pub flags: u64,
}

const CAP_SYS_ADMIN: u64 = 1 << 21;
const CAP_SYS_MODULE: u64 = 1 << 16;
const CAP_SYS_PTRACE: u64 = 1 << 19;
const CAP_NET_ADMIN: u64 = 1 << 12;
const CAP_NET_RAW: u64 = 1 << 13;
const CAP_SYS_CHROOT: u64 = 1 << 18;
const CAP_SYS_RAWIO: u64 = 1 << 17;
const CAP_SYS_BOOT: u64 = 1 << 22;
const CAP_DAC_READ_SEARCH: u64 = 1 << 2;
const CAP_SYSLOG: u64 = 1 << 34;
const CAP_ANY: u64 = CAP_SYS_ADMIN
    | CAP_SYS_MODULE
    | CAP_SYS_PTRACE
    | CAP_NET_ADMIN
    | CAP_NET_RAW
    | CAP_SYS_CHROOT
    | CAP_SYS_RAWIO
    | CAP_SYS_BOOT
    | CAP_DAC_READ_SEARCH
    | CAP_SYSLOG;

impl Test for CapTest {
    fn name(&self) -> String {
        "whether any dangerous capability bits are available".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = CapResult { flags: 0 };

        if let Ok(stat) = read_file_as_tuples("/proc/self/status") {
            if stat.contains_key("CapAmb") {
                if let Ok(flags) = u64::from_str_radix(stat["CapAmb"].as_str(), 16) {
                    result.flags |= flags;
                }
            }

            if stat.contains_key("CapEff") {
                if let Ok(flags) = u64::from_str_radix(stat["CapEff"].as_str(), 16) {
                    result.flags |= flags;
                }
            }
        }

        Ok(Box::new(result))
    }
}

impl TestResult for CapResult {
    fn success(&self) -> bool {
        self.flags & CAP_ANY == 0
    }

    fn explain(&self) {
        if self.success() {
            println!("  + No dangerous capability bits detected.");
            return;
        }

        let cap_names = HashMap::from([
            (CAP_SYS_ADMIN, "CAP_SYS_ADMIN"),
            (CAP_SYS_MODULE, "CAP_SYS_MODULE"),
            (CAP_SYS_PTRACE, "CAP_SYS_PTRACE"),
            (CAP_NET_ADMIN, "CAP_NET_ADMIN"),
            (CAP_NET_RAW, "CAP_NET_RAW"),
            (CAP_SYS_CHROOT, "CAP_SYS_CHROOT"),
            (CAP_SYS_RAWIO, "CAP_SYS_RAWIO"),
            (CAP_SYS_BOOT, "CAP_SYS_BOOT"),
            (CAP_DAC_READ_SEARCH, "CAP_DAC_READ_SEARCH"),
            (CAP_SYSLOG, "CAP_SYSLOG"),
        ]);

        for k in cap_names.keys() {
            if self.flags & k == *k {
                println!("  - {} found", cap_names[k]);
            }
        }
    }

    fn as_string(&self) -> String {
        if self.success() {
            return "no".to_string();
        }

        "yes".to_string()
    }
}
