//! 🧾️ Frame transaction and boot runtime.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_app_runtime::{AppFrameAfterChrome, AppFrameBuild, AppHandle, AppMailbox, AppRuntime, FrameBuildBoundaryStep, FrameBuildCursor, FrameFinishBoundaryStep, FrameFinishCursor, discard_frame_input_candidate, frame_child_step, frame_inline_gate, frame_inline_progress, frame_receive_close, frame_received_child, frame_unpriced_child, step_world3d_camera_fit_after_snapshot, world3d_ingest_trace, world3d_interaction_trace};
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners, FrameDirectives, FrameInputActionStep, WORLD3D_DEADLINE_ID_BYTES, transfer_frame_input_action};
use semio_framework_os_renderer_wgpu_frame_presentation::{AppPresentStallWatch, AppPresenter};
use semio_framework_os_renderer_wgpu_runtime_mailbox::{EFFECT_STORM_BUDGET, RuntimeApply, RuntimeMailbox, RuntimePresentationWitness};
use semio_framework_os_renderer_wgpu_shell_input::PointerCapture;
use semio_framework_os_renderer_wgpu_shell_prefs::shell_language_axes;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use infinite_world::world::{World3dSceneBridgeStep, World3dSnapshotApplyStep, WorldDrawRebuildStep, WorldInteractionAuthorityStep, WorldInteractionIntent, close_world3d_draw_rebuild_step, enqueue_world3d_event, retire_cancelled_world3d_asset_step, step_world3d_draw_rebuild, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, world3d_hover_clear_is_owed, world3d_interaction_front_generation};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, appearance_is_dark, frame_latency, log_debug};
use semio_framework_os_renderer_wgpu_program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_program_bridge::parse_plugin_entries;
use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;
use ui_gpu::GpuContext;
use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_text::{FontAtlas, fetch_font_bytes};
use semio_framework_ui_wgpu_input::{InputState, PointerModifiers};
use semio_framework_ui_theme::Theme;
use winit::window::Window;
use semio_framework_os_renderer_wgpu_icon_host as icon_atlas;
use semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;

#[path = "../🧊️renderer/🧾️frame-build/🧮️frame-transaction/🦀️.rs"]
mod frame_transaction;
pub use frame_transaction::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
