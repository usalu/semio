//! 🔁️ The plugin exchange function of the plugin runtime.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;

#[path = "🔀️plugin-exchange/🦀️.rs"]
pub mod exch_plugin_exchange;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
