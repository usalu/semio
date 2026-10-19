//! 🥝️ Companion test crate tests-16 of the plugin SDK partition: hosts the test roots `plugin_builder_contract_tests_3`, `receipt_tests_host`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🦀️.rs"]
pub mod plugin_builder_contract_tests_3;
#[cfg(test)]
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🎟️child-receipt/🦀️.rs"]
pub mod receipt_tests_host;

#[cfg(test)]
#[path = "../../🧪️tests/🧬️mutation-fixtures-job-close/🦀️.rs"]
pub(crate) mod job_close_fixture;
