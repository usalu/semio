//! 🎪️ OS host and winit application handler.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_app_runtime::AppMailbox;
use semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState;
use semio_framework_os_renderer_wgpu_frame_job::{frame_authority, frame_job};
use semio_framework_os_renderer_wgpu_frame_job::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT;
use semio_framework_os_renderer_wgpu_frame_job::frame_job::FrameBuildHandle;
use semio_framework_os_renderer_wgpu_frame_presentation::{AppPresentStep, AppPresenter};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_frame_transaction::boot_runtime;
use semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimeApply;
use semio_framework_os_renderer_wgpu_shell_window_actions::note_host_window_blur;
use infinite_world::world::WorldDynamicFault;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{HostAppearance, app_now_ms, boot_window_title, host_appearance, log_debug, log_debug_diagnostic, set_host_appearance, spawn_app_task};
use semio_framework_os_renderer_wgpu_host_plumbing::{deadlines, input_wire, render_snapshot, surface_lane};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_plumbing::native_caller;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::native_accessibility;
use std::sync::{Arc, OnceLock};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use ui_wgpu::wgpu::{SemioCursor, Theme};
use winit::window::Window;
use semio_framework_os_renderer_wgpu_media_slots as media_slots;

#[path = "../🏠️os-host/🦀️.rs"]
pub mod os_host;
#[path = "../🪟️winit-app/🦀️.rs"]
pub mod winit_app;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
