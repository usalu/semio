//! 🧶️ Companion test crate tests-04 of the plugin SDK partition: hosts the test roots `app_panel_kit_tests`, `owned_media_error_tests_host`, `testing_laws_unit_1` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🪝️mounts/🧵️wp7-instance-tier2/🧪️test-hosts/⚖️testing-laws/🧱️unit-1/🦀️.rs"]
pub mod testing_laws_unit_1;
