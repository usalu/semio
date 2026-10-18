//! 🥥️ Companion test crate tests-15 of the plugin SDK partition: hosts the test roots `testing_laws_unit_4`, `builder_test_hosts`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⚖️testing-laws/🧱️unit-4/🦀️.rs"]
pub mod testing_laws_unit_4;
#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/🔨️builder/🦀️.rs"]
pub mod builder_test_hosts;
