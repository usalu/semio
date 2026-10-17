//! 🥭️ Companion test crate tests-13 of the plugin SDK partition: hosts the test roots `job_tool_run_host_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/👷️job/🧱️unit-3/🦀️.rs"]
pub mod job_tool_run_host_1;
