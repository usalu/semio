//! 🫑️ Companion test crate tests-07 of the plugin SDK partition: hosts the test roots `gesture_laws`, `folder_reload_route` (part 3).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🧪️tests/🧪️gesture/🦀️.rs"]
pub mod gesture_laws;
#[cfg(test)]
#[path = "../../🧪️tests/🧪️folder-reload-route/🦀️.rs"]
pub mod folder_reload_route;
