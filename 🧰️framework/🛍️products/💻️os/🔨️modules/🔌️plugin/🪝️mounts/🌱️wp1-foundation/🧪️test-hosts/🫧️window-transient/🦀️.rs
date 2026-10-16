//! 🫧️ Test host of `window_transient`: the laws that name the facade publication fixtures and the exported transient macros.
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use super::window_config::registry::WindowRegistry;
use super::window_config::registry::fits;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_plugin_ownership::transient_publication::transient_store_disposer;
use crate::protocol;
use crate::store;
use semio_framework::Fault;
use semio_framework::ViewModel;
use std::sync::Arc;
pub use crate::window_transient::*;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🦀️.rs"]
mod retained_window_input_tests;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs"]
mod transient_root_tests;
