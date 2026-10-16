//! 🪢️ The wasip2 WIT bridge of the plugin reactor.

extern crate semio_framework_os_kernel as store;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[path = "🪢️bridge/🦀️.rs"]
pub mod wit_bridge;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
