//! 🌆️ Companion test bin of the wgpu renderer: windows and panels (2).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_os_renderer_wgpu_shell_chrome_state::{CHROME_TOOLTIP_DELAY_MS, ChromeTooltipHover};
use semio_framework_os_renderer_wgpu_shell_dock::{TREE_WINDOW_REPORT_DEBOUNCE_MS, TreeWindowScheduler, tutorial_resolve_gesture_point};
use semio_framework_os_renderer_wgpu_shell_input::OverlayState;
use semio_framework_os_renderer_wgpu_shell_prefs::DockTabNode;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{TutorialCameraConverge, TutorialMode, TutorialRuntime, orbit_to_tutorial_camera, tutorial_camera_to_orbit};
use semio_framework_os_renderer_wgpu_shell_view_paint::{tutorial_scrub_progress, tutorial_scrub_target_ms};
use semio_framework_os_renderer_wgpu_shell_window_actions::{WindowActionsCtx, scope_action_to_window, tutorial_advance_playhead, tutorial_apply_ui_change_to_shell, tutorial_apply_ui_snapshot, tutorial_camera_pose_close, tutorial_capture_ui_snapshot};
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, Rect};
use ui_wgpu::wgpu::tree_window::TreeWindowRequest;
use winit::window::Window;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️tree-windows/🦀️.rs"]
mod tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️action-window-scope/🦀️.rs"]
mod action_window_scope_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs"]
mod tutorial_tests;
