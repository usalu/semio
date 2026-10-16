//! 🕰️ Time travel panel view, pointer and panel of the plugin SDK.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;

#[path = "🛬️lifted/🎮️command/🦀️.rs"]
pub mod command_history_panel_lift;
#[path = "🔢️pointer/🦀️.rs"]
pub mod panel_pointer;
#[path = "🖼️sections/🦀️.rs"]
pub mod panel_sections;

#[doc(hidden)]
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub mod tests;
