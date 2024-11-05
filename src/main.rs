pub mod cap;
pub mod containerd;
pub mod docker;
pub mod mmap;
pub mod oci;
pub mod procmask;
pub mod root;
pub mod rootns;
pub mod seccomp;
pub mod util;
pub mod yama;

use std::env;
use std::error::Error;
use std::process;

use self::cap::CapTest;
use self::docker::DockerTest;
use self::containerd::ContainerDTest;
use self::mmap::MmapRWXTest;
use self::oci::OCITest;
use self::procmask::ProcMaskTest;
use self::root::RootTest;
use self::rootns::RootNSTest;
use self::seccomp::SeccompTest;
use self::yama::YamaTest;

use anyhow::Result;

fn banner() {
    println!("Am I Isolated version {}.  Copyright 2024 Edera Inc.\n", env!("CARGO_PKG_VERSION"));
}

pub trait TestResult {
    fn success(&self) -> bool;
    fn explain(&self) -> String;
    fn as_string(&self) -> String;
    fn fault_code(&self) -> String;
}

pub trait TestError: Error {
}

pub trait Test {
    fn name(&self) -> String;
    fn run(&self) -> Result<Box<dyn TestResult>, ()>;
}

fn usage() {
    println!("usage: am-i-isolated [--only-failed-tests] [--help]");
    println!("");
    println!("  --only-failed-tests       show only failed tests");
    println!("  --help                    show help message");
    process::exit(1);
}

fn main() {
    let mut show_passing = true;
    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg == "--only-failed-tests" {
            show_passing = false;
        } else if arg == "--help" {
            usage();
        }
    }

    banner();

    let tests: Vec<Box<dyn Test>> = vec![
        Box::new(OCITest {}),
        Box::new(DockerTest {}),
        Box::new(ContainerDTest {}),
        Box::new(MmapRWXTest {}),
        Box::new(ProcMaskTest {}),
        Box::new(RootTest {}),
        Box::new(SeccompTest {}),
        Box::new(CapTest {}),
        Box::new(RootNSTest {}),
        Box::new(YamaTest {}),
    ];

    for test in &tests {
        let result = test.run().expect("failed to run test");
        if !result.success() {
            println!("\x1B[31m{}: [{}] {}\x1B[0m", test.name(), result.fault_code(), result.explain());
        } else if show_passing {
            println!("\x1B[32m{}: {}\x1B[0m", test.name(), result.as_string());
        }
    }
}
