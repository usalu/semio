//! 🔬️ Native socket probe, smoke runner and sqlite probe.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate semio_framework_os_kernel as dsl;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
use semio_framework_os_renderer_wgpu_app_runtime::ICON_ATLAS_RUNTIME;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_prefs::shell_language_axes;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_sync::Phase;
use semio_framework_os_renderer_wgpu_shell_window_actions::WindowActionsCtx;
use semio_framework_job::RetainedCloneGrant;
use semio_framework_os_renderer_wgpu_host_base::{renderer_worker_pool, runtime_assets};
use semio_framework_os_renderer_wgpu_program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins;
use std::collections::HashMap;
use std::sync::Arc;
use semio_framework_ui_draw_contract::IconAtlas;

#[path = "../🧊️renderer/🔬️native-socket-probe/🦀️.rs"]
mod native_socket_probe;
pub use native_socket_probe::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
