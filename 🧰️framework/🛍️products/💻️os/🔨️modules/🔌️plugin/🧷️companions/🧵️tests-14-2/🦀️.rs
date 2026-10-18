//! 🧵️ Companion test crate tests-14 of the plugin SDK partition: hosts the test roots `app_window_kits_tests`, `plugin_builder_contract_tests_4` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-4/🦀️.rs"]
pub mod plugin_builder_contract_tests_4;
