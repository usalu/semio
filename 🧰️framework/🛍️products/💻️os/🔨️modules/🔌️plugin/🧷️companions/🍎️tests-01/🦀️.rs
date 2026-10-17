//! 🍎️ Companion test crate tests-01 of the plugin SDK partition: hosts the test roots `retained_command_test_hosts`, `runtime_instances_test_hosts`, `time_travel_turns_test_hosts`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs"]
pub mod retained_command_test_hosts;

