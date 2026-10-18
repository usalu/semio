//! 🥑️ Companion test crate tests-18 of the plugin SDK partition: hosts the test roots `declarations_fixture_tests`, `declaration_fixture_mutations`, `artifact_admission_tests`.
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
#[path = "../../🪝️mounts/📜️wp2-declaration/🧪️hosts/🔬️declarations-fixture/🦀️.rs"]
pub mod declarations_fixture_tests;
#[cfg(test)]
#[path = "../../🧪️tests/🛰️declaration-channels/🦀️.rs"]
pub mod declaration_fixture_mutations;
#[cfg(test)]
#[path = "../../🏗️builder/🧪️tests/🪪️artifact-admission/🦀️.rs"]
pub mod artifact_admission_tests;
