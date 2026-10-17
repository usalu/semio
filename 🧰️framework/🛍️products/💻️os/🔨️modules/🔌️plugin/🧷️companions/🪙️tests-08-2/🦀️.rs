//! 🪙️ Companion test crate tests-08 of the plugin SDK partition: hosts the test roots `bounded_initializer_laws`, `runtime_ops_test_hosts`, `reactor_surfaces_test_hosts`, `plugin_builder_contract_tests_1` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/⚙️runtime-ops/🦀️.rs"]
pub mod runtime_ops_test_hosts;
#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/🖼️reactor-surfaces/🦀️.rs"]
pub mod reactor_surfaces_test_hosts;
