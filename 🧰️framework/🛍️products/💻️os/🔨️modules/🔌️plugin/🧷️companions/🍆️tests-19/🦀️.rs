//! 🍆️ Companion test crate tests-19 of the plugin SDK partition: hosts the test roots `window_mutation_tests`, `window_config_tests`, `window_transient_tests`.
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
#[path = "../../🪝️mounts/🌱️wp1-foundation/🧪️test-hosts/📬️window-mutation/🦀️.rs"]
pub mod window_mutation_tests;
#[cfg(test)]
#[path = "../../🪝️mounts/🌱️wp1-foundation/🧪️test-hosts/🎚️window-config/🦀️.rs"]
pub mod window_config_tests;
#[cfg(test)]
#[path = "../../🪝️mounts/🌱️wp1-foundation/🧪️test-hosts/🫧️window-transient/🦀️.rs"]
pub mod window_transient_tests;
