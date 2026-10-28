//! 🎚️ Test host of `window_config`: the laws that name the facade test fixtures run in the facade test build.
use semio_framework_plugin_window_config::retained::ErasedWindowConfigPackLoad;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework_value::FactoryBoxedValue;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_window_config::registry::WindowRegistry;
pub use semio_framework_plugin_window_config::*;
pub use semio_framework_plugin_window_config::retained::*;
#[path = "../../../../🪟️window/🎚️config/🧬️preparation/🧪️tests/🦀️.rs"]
mod original_preparation_tests;
#[doc(hidden)]
#[path = "../../../../🪟️window/🎚️config/🧪️tests/🪟️retained-window-config/🦀️.rs"]
pub mod tests;
#[path = "../../../../🪟️window/🎚️config/🧪️tests/📷️camera-owner/🦀️.rs"]
mod camera_owner;
#[path = "../../../../🪟️window/🎚️config/🧪️tests/🪪️pack-identity/🦀️.rs"]
mod pack_identity_tests;
#[path = "../../../../🪟️window/🎚️config/🧪️tests/📥️retained-pack-load/🦀️.rs"]
mod retained_pack_load_tests;
#[path = "🧬️retained/🦀️.rs"]
mod retained_tests;
#[path = "../../../../🪟️window/🎚️config/🧬️preparation/🔁️replace/🧪️tests/🦀️.rs"]
mod replace_edit_tests;
#[path = "🗂️registry/🦀️.rs"]
mod registry_tests;
