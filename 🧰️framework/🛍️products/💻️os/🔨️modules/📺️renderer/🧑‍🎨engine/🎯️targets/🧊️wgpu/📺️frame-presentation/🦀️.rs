//! 📺️ App presenter, present stall watch and surface context.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_app_runtime::{AppFramePresentation, AppMailbox};
use semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState;
use semio_framework_os_renderer_wgpu_runtime_mailbox::{MailboxRuntime, PresenterInteractionStep, RuntimePresentationAuthority, RuntimeRasterOperationAuthority};
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use infinite_world::world::{begin_world3d_dynamic_retirement, step_world3d_dynamic_retirement, world3d_dynamic_retirement_terminal_is_empty};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, frame_latency, log_debug, log_debug_once_per_transition};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_base::{RENDERER_IO_WAKE, pump_renderer_io_sessions};
use semio_framework_os_renderer_wgpu_host_plumbing::surface_lane;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use ui_gpu::GpuContext;
use ui_wgpu::wgpu::{SemioCursor, apply_window_cursor};
use winit::window::Window;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;
use semio_framework_os_renderer_wgpu_media_slots as media_slots;

#[path = "../🧊️renderer/🖥️frame-presentation/🦀️.rs"]
mod frame_presentation;
pub use frame_presentation::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
