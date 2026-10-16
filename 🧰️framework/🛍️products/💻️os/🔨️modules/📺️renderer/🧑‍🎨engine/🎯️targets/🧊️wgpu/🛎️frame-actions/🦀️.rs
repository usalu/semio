//! 🛎️ Frame action envelopes, receipts, owners and deferred cursors.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_shell_input::{PointerCapture, PointerHitOwner};
use semio_framework_os_renderer_wgpu_shell_prefs::dispatch_refusal_discards_input_draft;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use infinite_world::world::{WorldInteractionIntent, enqueue_world3d_event, world3d_hover_is_published};
use semio_framework_job::RetainedCloneProgress;
use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, frame_latency, log_debug};
use semio_framework_os_renderer_wgpu_host_plumbing::input_wire;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use ui_wgpu::wgpu::{ActionDescriptor, InputState, KeyAction, PointerModifiers, Theme};

#[path = "../🧊️renderer/🛎️frame-actions/🦀️.rs"]
mod frame_actions;
pub use frame_actions::*;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️fixtures/🦀️.rs"]
mod fixtures;
#[cfg(any(test, feature = "testkit"))]
pub use fixtures::collect_fixture_actions;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
