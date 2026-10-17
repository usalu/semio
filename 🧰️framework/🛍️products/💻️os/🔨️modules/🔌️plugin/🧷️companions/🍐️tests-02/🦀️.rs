//! 🍐️ Companion test crate tests-02 of the plugin SDK partition: hosts the test roots `app_builder_tests_host`, `child_test_hosts`, `publication_mounted_test_hosts`, `tool_run_test_hosts`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🏗️app-builder/🦀️.rs"]
pub mod app_builder_tests_host;
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/🪆️child/🦀️.rs"]
pub mod child_test_hosts;
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/🏇️publication-mounted/🦀️.rs"]
pub mod publication_mounted_test_hosts;

