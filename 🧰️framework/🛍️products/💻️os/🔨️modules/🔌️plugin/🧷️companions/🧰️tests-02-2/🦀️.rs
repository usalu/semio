//! 🧰️ Companion test crate tests-02 of the plugin SDK partition: hosts the test roots `app_builder_tests_host`, `child_test_hosts`, `publication_mounted_test_hosts`, `tool_run_test_hosts` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as vcs;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[cfg(test)]
#[path = "../../🪝️mounts/⏯️wp5-features/🧪️test-hosts/⏯️tool-run/🦀️.rs"]
pub mod tool_run_test_hosts;
