//! 🏃️ App runtime, interaction state, frame engine packets and build cursors.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners, FrameDeferredCursor, FrameDirectives, SHELL_SYNC_PUMP_INTERVAL_MS};
use semio_framework_os_renderer_wgpu_runtime_mailbox::{MailboxRuntime, RuntimeApply, RuntimeCompletion, RuntimeHandle, RuntimeMailbox, RuntimePresentationWitness};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_runtime_mailbox::FrameMaintenanceRefusal;
use semio_framework_os_renderer_wgpu_shell_docs::resolve_theme_for_ids;
use semio_framework_os_renderer_wgpu_shell_input::PresentedInputCandidateWitness;
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_session::SessionCtx;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, ShellChromeFrameCursor};
use semio_framework_os_renderer_wgpu_shell_window_actions::WindowActionsCtx;
use infinite_world::world::{WorldInteractionAuthorityStep, step_world3d_camera_fit};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, appearance_is_dark, log_debug, log_debug_diagnostic, spawn_app_task};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_base::{RendererIoHandle, submit_renderer_io};
use semio_framework_os_renderer_wgpu_host_plumbing::deadlines;
use semio_framework_os_renderer_wgpu_program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use semio_framework_ui_wgpu_hosting::cursor::{CursorDragState, SemioCursor, resolve_semio_cursor};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_kernel_runtime as kernel_runtime;
use semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;

#[path = "../🧊️renderer/🧠️app-runtime/🦀️.rs"]
mod app_runtime;
pub use app_runtime::*;
#[path = "../🧊️renderer/🧾️frame-build/🦀️.rs"]
mod frame_build;
pub use frame_build::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
