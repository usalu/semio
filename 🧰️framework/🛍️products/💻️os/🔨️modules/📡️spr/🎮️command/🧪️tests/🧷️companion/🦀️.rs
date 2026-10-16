//! 🧷️ Companion tests of os-command: the command protocol unit tests, the mutation payload law and the registry and mutation-law fixtures.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_command::*;
use semio_framework_value_derive::{FromValue, ToValue};

#[path = "../📔️registry/🦀️.rs"]
mod registry_fixture;

#[path = "../🧬️mutation-laws/🦀️.rs"]
mod mutation_laws_fixture;

#[path = "../🔬️unit/🦀️.rs"]
mod tests;

#[path = "../🧪️mutation-payload/🦀️.rs"]
mod mutation_payload_tests;
