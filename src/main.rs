pub mod cap;
pub mod containerd;
pub mod dirtypipe;
pub mod docker;
pub mod mmap;
pub mod oci;
pub mod procmask;
pub mod root;
pub mod rootns;
pub mod seccomp;
pub mod util;
pub mod virtualized;
pub mod yama;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::process;

use self::cap::CapTest;
use self::containerd::ContainerDTest;
use self::dirtypipe::DirtyPipeTest;
use self::docker::DockerTest;
use self::mmap::MmapRWXTest;
use self::oci::OCITest;
use self::procmask::ProcMaskTest;
use self::root::RootTest;
use self::rootns::RootNSTest;
use self::seccomp::SeccompTest;
use self::virtualized::VirtualizedTest;
use self::yama::YamaTest;

use anyhow::Result;

fn banner() {
    println!("🧰 Am I Isolated from Edera\n",);
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TestCategory {
    High,
    Medium,
    Low,
}

impl TestCategory {
    fn as_name(&self) -> &str {
        match self {
            TestCategory::High => "High Priority",
            TestCategory::Medium => "Medium Priority",
            TestCategory::Low => "Low Priority",
        }
    }

    fn as_emoji(&self) -> &str {
        match self {
            TestCategory::High => "🔥",
            TestCategory::Medium => "😬",
            TestCategory::Low => "🤔",
        }
    }

    fn as_fail_emoji(&self) -> &str {
        match self {
            TestCategory::High => "❌",
            TestCategory::Medium => "❌",
            TestCategory::Low => "⚠️",
        }
    }
}

pub trait TestResult {
    fn success(&self) -> bool;
    fn explain(&self) -> String;
    fn as_string(&self) -> String;
    fn fault_code(&self) -> String;
}

pub trait TestError: Error {}

pub trait Test {
    fn name(&self) -> String;
    fn run(&self) -> Result<Box<dyn TestResult>, ()>;
    fn category(&self) -> TestCategory;
}

fn usage() {
    println!("usage: am-i-isolated [--only-failed-tests] [--help]");
    println!();
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
        Box::new(VirtualizedTest {}),
        Box::new(OCITest {}),
        Box::new(DockerTest {}),
        Box::new(ContainerDTest {}),
        Box::new(DirtyPipeTest {}),
        Box::new(MmapRWXTest {}),
        Box::new(ProcMaskTest {}),
        Box::new(RootTest {}),
        Box::new(SeccompTest {}),
        Box::new(CapTest {}),
        Box::new(RootNSTest {}),
        Box::new(YamaTest {}),
    ];

    let all_results = tests
        .iter()
        .map(|test| {
            let result = test.run().unwrap_or_else(|_| panic!("test '{}' failed", test.name()));
            (test, result)
        })
        .collect::<Vec<_>>();

    let mut categorized_results: BTreeMap<
        TestCategory,
        Vec<(&Box<dyn Test>, Box<dyn TestResult>)>,
    > = BTreeMap::new();
    for (test, result) in all_results {
        let category = test.category();
        categorized_results
            .entry(category)
            .or_default()
            .push((test, result));
    }

    for (_, tests) in categorized_results.iter_mut() {
        tests.sort_by(|(_, result_a), (_, result_b)| {
            if result_a.success() == result_b.success() {
                Ordering::Equal
            } else if result_a.success() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }

    for (i, (category, tests)) in categorized_results.iter().enumerate() {
        println!("{} {}:", category.as_emoji(), category.as_name());

        for (test, result) in tests {
            if result.success() && show_passing {
                println!("  ✅ {} test passed: {}", test.name(), result.explain());
            } else {
                println!(
                    "  {} {} test failed: {} [{}]",
                    category.as_fail_emoji(),
                    test.name(),
                    result.explain(),
                    result.fault_code()
                );
            }
        }

        if i != categorized_results.len() - 1 {
            println!();
        }
    }
}
