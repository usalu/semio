//! 🍋️ Companion test crate tests-04 of the plugin SDK partition: hosts the test roots `app_panel_kit_tests`, `owned_media_error_tests_host`, `testing_laws_unit_1`.
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as dsl;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]

#[path = "../../🧪️tests/🔬️app-panel-kit/🦀️.rs"]
pub mod app_panel_kit_tests;
#[cfg(test)]
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🎞️owned-media-error/🦀️.rs"]
pub mod owned_media_error_tests_host;

