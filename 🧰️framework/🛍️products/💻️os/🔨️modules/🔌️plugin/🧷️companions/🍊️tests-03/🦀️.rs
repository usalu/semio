//! 🍊️ Companion test crate tests-03 of the plugin SDK partition: hosts the test roots `media_export_credit_tests_host`, `media_publication_tests_host`, `document_media_test_hosts`, `reactor_core_test_hosts`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;

#[cfg(test)]

#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/📤️export-credit/🦀️.rs"]
pub mod media_export_credit_tests_host;
#[cfg(any(test, feature = "media-turn-testing"))]
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/📬️media-publication/🦀️.rs"]
pub mod media_publication_tests_host;
#[cfg(any(test, feature = "media-turn-testing"))]
#[path = "../../🪝️mounts/👥️wp3-owners/🧪️test-hosts/🎞️document-media/🦀️.rs"]
pub mod document_media_test_hosts;

