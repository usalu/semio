//! 📨️ Generic runtime mailbox, presentation and raster authorities, native asset transport and runtime apply.
//!
//! 🧭️ The files WP3 staged for this crate (`wp3-tools/crate-map.json`) are mounted here from their place in the renderer tree; the renderer root re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_asset_authority::{RendererAssetFetchOwner, RendererAssetSealStep, close_renderer_asset_step, finish_renderer_asset, renderer_asset_io, reserve_renderer_asset_response, retire_cancelled_renderer_asset_step, return_renderer_asset, seal_renderer_asset_response, take_next_completed_renderer_asset_step, take_next_renderer_asset};
use semio_framework_os_renderer_wgpu_asset_decode::{RendererAssetDecodeInvocation, RendererAssetDecodeResult, RendererAssetDecodeSession, RendererAssetDecodeTurn, RendererAssetDecodeTurnKind, RendererAssetProbe, RendererAssetProbePhase};
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners, FrameActionReceiptOwner, FrameBuildInputs, FrameDeferredCursor, FrameDeferredWork, RuntimeDispatchCursor, RuntimeDispatchSource, dispatch_normalized_event, settle_renderer_action_receipt};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_frame_actions::{FrameExecutionOwnerCell, FrameExecutionRegistry, FrameMaintenanceAuthority, frame_maintenance_terminal_fault};
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ChromeViewCtx;
use infinite_world::world::{WORLD_ASSET_RESPONSE_PAGE_BYTES, WorldAssetMetadataId, WorldAssetRequestKind, WorldAssetRequestToken, WorldAssetResponsePage, WorldDynamicFault, apply_world3d_terrain_tile_bytes, collect_world3d_asset_bytes, finish_world3d_asset, mark_world3d_asset_miss, publish_world3d_asset_mesh, reserve_world3d_asset_response, return_world3d_asset, seal_world3d_asset_response, take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_asset_cancellation_requested};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use infinite_world::world::{DecodedReferenceImage, apply_decoded_reference_raster, world3d_reference_url_is_current};
#[cfg(not(target_arch = "wasm32"))]
use infinite_world::world::WORLD_ASSET_RESPONSE_BYTE_CAPACITY;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceToken;
use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, frame_latency, log_debug, log_debug_diagnostic, log_debug_once_per_transition, mesh_assets, renderer_worker_pool, spawn_app_task};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_base::run_renderer_io;
use semio_framework_os_renderer_wgpu_host_plumbing::deadlines;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use ui_wgpu::wgpu::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode, SceneRasterWriter};
use semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;
use semio_framework_os_renderer_wgpu_program_bridge as program_bridge;

#[path = "../🧊️renderer/🎨️presentation-raster/🦀️.rs"]
mod presentation_raster;
pub use presentation_raster::*;
#[path = "../🧊️renderer/📮️mailbox-impl/🦀️.rs"]
mod mailbox_impl;
pub use mailbox_impl::*;
#[path = "../🧊️renderer/🛎️frame-actions/🔁️runtime-apply/🦀️.rs"]
mod runtime_apply;
pub use runtime_apply::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
