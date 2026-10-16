//! 🍰️ Companion test bin of the wgpu renderer: input and pointer (3).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_os_renderer_wgpu_shell_dock::ShellKeyboardStop;
use semio_framework_os_renderer_wgpu_shell_input::{ContextMenuItem, ContextMenuState, context_menu_path_for_ordinal};
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_prefs::DockTabNode;
use semio_framework_os_renderer_wgpu_shell_session::SessionCtx;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_view_overlays::{context_menu_level_width, context_menu_submenu_open, render_context_menu_level};
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::native_accessibility;
use semio_framework_ui_locale::{Locale, Terminology};
use serde_json::Value;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, KeyAction, PointerModifiers, Rect, Theme};
use winit::window::Window;

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-context-menu-keyboard/🦀️.rs"]
mod context_menu_keyboard_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⌨️native-keyboard-ring/🦀️.rs"]
mod native_keyboard_ring_tests;
