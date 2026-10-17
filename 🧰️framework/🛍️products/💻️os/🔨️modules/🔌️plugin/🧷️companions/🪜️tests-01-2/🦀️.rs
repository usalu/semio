//! 🪜️ Companion test crate tests-01 of the plugin SDK partition: hosts the test roots `retained_command_test_hosts`, `runtime_instances_test_hosts`, `time_travel_turns_test_hosts` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/📦️runtime-instances/🦀️.rs"]
pub mod runtime_instances_test_hosts;
#[cfg(test)]
#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⏳️time-travel-turns/🦀️.rs"]
pub mod time_travel_turns_test_hosts;
