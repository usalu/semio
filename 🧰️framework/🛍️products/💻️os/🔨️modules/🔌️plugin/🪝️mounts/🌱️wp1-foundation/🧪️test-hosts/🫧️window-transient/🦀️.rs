//! 🫧️ Test host of `window_transient`: the laws that name the facade publication fixtures and the exported transient macros.
use semio_framework_plugin_window_transient::window_transient::TypedWindowTransientStoreOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientAuthority;
use semio_framework_plugin_window_transient::window_transient::WindowTransientMutation;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerBundle;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_framework_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_framework_plugin_window_transient::window_transient::WindowTransientStore;
use semio_framework_plugin_window_transient::window_transient::ErasedWindowTransientPublication;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_window_config_registry::WindowRegistry;
use semio_framework_plugin_window_config_registry::fits;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_framework_plugin_ownership::transient_publication::transient_store_disposer;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
use semio_framework::Fault;
use semio_framework::ViewModel;
use std::sync::Arc;
pub use semio_framework_plugin_window_transient::window_transient::*;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🦀️.rs"]
mod retained_window_input_tests;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs"]
mod transient_root_tests;
