//! 🍈️ Companion test crate tests-10 of the plugin SDK partition: hosts the test roots `plugin_builder_contract_tests_2`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-2/🦀️.rs"]
pub mod plugin_builder_contract_tests_2;
