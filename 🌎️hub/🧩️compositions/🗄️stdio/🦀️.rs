//! 🔌️ Hub Stdio deployment composition over independently compiled artifact packages.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[cfg(feature = "component-app-assembly")]
#[path = "🔌️plugin/🦀️.rs"]
pub mod plugin;
#[cfg(feature = "component-app-assembly")]
pub use plugin::plugin;
#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!({ let grant = semio_framework_plugin::app::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }, plugin, plugin::StdioApps);

#[path = "🔌️plugin/📇️catalog/🦀️.rs"]
pub mod catalog;

#[cfg(feature = "full-artifact-catalog")]
#[path = "🛂️manifest/🦀️.rs"]
pub mod manifest;
