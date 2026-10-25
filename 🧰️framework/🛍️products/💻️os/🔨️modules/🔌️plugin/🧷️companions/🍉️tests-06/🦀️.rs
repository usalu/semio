//! 🍉️ Companion test crate tests-06 of the plugin SDK partition: hosts the test roots `tool_run_support_1`, `extension_test_hosts`, `time_travel_support_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "../../🧪️tests/🔬️tool-run/🦀️.rs"]
pub mod tool_run_support_1;
#[path = "../../🧪️tests/🧪️time-travel/🦀️.rs"]
pub mod time_travel_support_1;

