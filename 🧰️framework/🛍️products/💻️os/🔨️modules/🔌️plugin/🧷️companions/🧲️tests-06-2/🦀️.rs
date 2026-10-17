//! 🧲️ Companion test crate tests-06 of the plugin SDK partition: hosts the test roots `tool_run_support_1`, `extension_test_hosts`, `time_travel_support_1` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧩️extension/🦀️.rs"]
pub mod extension_test_hosts;
