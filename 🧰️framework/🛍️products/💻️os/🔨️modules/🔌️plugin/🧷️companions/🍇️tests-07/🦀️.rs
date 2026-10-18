//! 🍇️ Companion test crate tests-07 of the plugin SDK partition: hosts the test roots `natural_file_lifecycle_tests`, `tool_run_turns_test_hosts`, `testing_laws_unit_3`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🧪️tests/📄️natural-file-lifecycle/🦀️.rs"]
pub mod natural_file_lifecycle_tests;
#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⏩️tool-run-turns/🦀️.rs"]
pub mod tool_run_turns_test_hosts;

