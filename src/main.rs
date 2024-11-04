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

use std::error::Error;

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
    println!("Am I Isolated version {}.  Copyright 2024 Edera Inc.", env!("CARGO_PKG_VERSION"));
}

pub trait TestResult {
    fn success(&self) -> bool;
    fn explain(&self);
    fn as_string(&self) -> String;
}

pub trait TestError: Error {
}

pub trait Test {
    fn name(&self) -> String;
    fn run(&self) -> Result<Box<dyn TestResult>, ()>;
}

fn main() {
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

    let mut passing = 0;
    for test in &tests {
        print!("\n* Checking {}... ", test.name());
        let result = test.run().expect("failed to run test");
        println!("{}", result.as_string());
        result.explain();
        if result.success() {
            passing += 1;
        }
    }

    println!("\nScore: {}/{}", passing, &tests.len());
}
