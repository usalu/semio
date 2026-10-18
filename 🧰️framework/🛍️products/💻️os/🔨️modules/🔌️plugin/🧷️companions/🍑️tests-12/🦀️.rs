//! 🍑️ Companion test crate tests-12 of the plugin SDK partition: hosts the test roots `job_test_hosts`, `plugin_builder_contract_tests_7`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/👷️job/🦀️.rs"]
pub mod job_test_hosts;

