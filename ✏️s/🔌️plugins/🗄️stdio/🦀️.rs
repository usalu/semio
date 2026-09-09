//! 🔌️ Stdio plugin composition over independently compiled artifact packages.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[cfg(feature = "component-app-assembly")]
#[path = "🔌️plugin/🦀️.rs"]
pub mod plugin;
#[cfg(feature = "component-app-assembly")]
pub use plugin::plugin;
#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, plugin::StdioApps);

#[path = "📇️registry/🦀️.rs"]
pub mod registry;

#[cfg(feature = "full-artifact-catalog")]
#[path = "🛂️manifest/🦀️.rs"]
pub mod manifest;
