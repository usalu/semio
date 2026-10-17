//! 🧱️ Test host of the window-config `retained` module.
use semio_framework_plugin_window_config::retained::RetainedStatePhase;
use semio_framework_plugin_window_config::retained::RetainedWindowConfigStateDecode;
use semio_framework_plugin_window_config::retained::RetainedWindowConfigTypedState;
use semio_framework_plugin_window_config::retained::ValueFrame;
use semio_framework_plugin_window_config::retained::controlled_demands;
use semio_framework_plugin_window_config::retained::ErasedWindowConfigPackLoad;
use semio_framework_os_kernel as store;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
pub use semio_framework_plugin_window_config::retained::*;
#[path = "../../../../../🪟️window/🎚️config/🧪️tests/🧬️physical-ownership/🦀️.rs"]
mod physical_ownership_tests;
