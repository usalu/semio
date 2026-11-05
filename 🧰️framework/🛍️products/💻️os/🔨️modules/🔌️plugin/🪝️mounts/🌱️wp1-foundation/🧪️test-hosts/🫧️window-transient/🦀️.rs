//! 🫧️ Test host of `window_transient`: the laws that name the facade publication fixtures and the exported transient macros.
use semio_plugin_window_transient::window_transient::TypedWindowTransientStoreOwner;
use semio_plugin_window_transient::window_transient::WindowTransientAuthority;
use semio_plugin_window_transient::window_transient::WindowTransientMutation;
use semio_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_plugin_window_transient::window_transient::WindowTransientOwnerBundle;
use semio_plugin_window_transient::window_transient::WindowTransientOwnerRegistry;
use semio_plugin_window_transient::window_transient::WindowTransientSnapshot;
use semio_plugin_window_transient::window_transient::WindowTransientStore;
use semio_plugin_window_transient::window_transient::ErasedWindowTransientPublication;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_plugin_window_config_registry::WindowRegistry;
use semio_plugin_window_config_registry::fits;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value::RetainedCloneProgress;
use semio_plugin_ownership::transient_publication::transient_store_disposer;
use semio_os_kernel as protocol;
use semio_os_kernel as store;
use semio_framework::Fault;
use semio_framework::ViewModel;
use std::sync::Arc;
pub use semio_plugin_window_transient::window_transient::*;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🪟️retained-window-input/🦀️.rs"]
mod retained_window_input_tests;
#[path = "../../../../🪟️window/🫧️transient/🧪️tests/🧪️transient-root/🦀️.rs"]
mod transient_root_tests;
#[path = "../../../../🪟️window/🫧️transient/🌱️registration/🧪️tests/🦀️.rs"]
mod original_registration_tests;
