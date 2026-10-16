//! 🪝️ The mutation protocol action adapter.

extern crate semio_framework_os_kernel as store;

#[path = "../🔀️dispatch/🦀️.rs"]
pub mod actions;

#[cfg(test)]
#[path = "../🔀️dispatch/🧪️tests/🔬️quick/🦀️.rs"]
pub mod actions_quick;
