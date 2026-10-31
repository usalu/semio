//! 🫒️ Companion test crate tests-05 of the plugin SDK partition: hosts the test root `reactor_turn_unit_3` (part 3).
//!
//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.

#[cfg(test)]
extern crate semio_framework_os_kernel as store;
#[cfg(test)]
extern crate semio_framework_os_kernel as protocol;
#[cfg(test)]
extern crate semio_framework_os_kernel as pack;

#[cfg(test)]
#[path = "../../🪝️mounts/⚛️wp4-reactor/🧪️test-hosts/🔂️reactor-turn/🧱️unit-3/🦀️.rs"]
pub mod reactor_turn_unit_3;
