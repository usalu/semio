//! 🍯️ Companion tests of os-directory (1 of 2): schema unit tests.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_directory::*;

#[path = "../../🧬️schema/🧪️tests/🔬️unit"]
pub mod schema {
    pub use semio_framework_os_directory::schema::*;
    use semio_framework_value_derive::ToValue;

    #[path = "🦀️.rs"]
    mod tests;
}
