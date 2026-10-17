//! 🪢️ Companion test crate tests-03 of the plugin SDK partition: hosts the test roots `media_export_credit_tests_host`, `media_publication_tests_host`, `document_media_test_hosts`, `reactor_core_test_hosts` (part 2).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]

#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/⚛️reactor-core/🦀️.rs"]
pub mod reactor_core_test_hosts;
