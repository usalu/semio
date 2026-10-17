//! 🍓️ Companion test crate tests-08 of the plugin SDK partition: hosts the test roots `bounded_initializer_laws`, `runtime_ops_test_hosts`, `reactor_surfaces_test_hosts`, `plugin_builder_contract_tests_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🌱️initialization/♻️retirement/🧪️tests/🦀️.rs"]
pub mod bounded_initializer_laws;
#[cfg(test)]
#[path = "../../🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1/🦀️.rs"]
pub mod plugin_builder_contract_tests_1;

