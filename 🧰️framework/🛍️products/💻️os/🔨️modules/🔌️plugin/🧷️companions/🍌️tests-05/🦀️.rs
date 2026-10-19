//! 🍌️ Companion test crate tests-05 of the plugin SDK partition: hosts the test roots `fixture_projection_retirement_tests`, `reactor_turn_test_hosts`, `plugin_runtime_plugin_builder_contract_support_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as dsl;

#[cfg(test)]

#[path = "../../🧪️tests/🌲️fixture-projection/🦀️.rs"]
pub mod fixture_projection_retirement_tests;
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs"]
pub mod plugin_runtime_plugin_builder_contract_support_1;


#[path = "../../🧪️tests/🧬️mutation-fixtures-job-close/🦀️.rs"]
pub(crate) mod job_close_fixture;
