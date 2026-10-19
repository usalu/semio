//! 🗂️ Engine surface registry, sync caches and retirement.
//!
//! The per-worker engine surface registry: node graph engines, the surface slots and their ids, tokens and live
//! freshness, the four per-kind sync caches, the terminal checks and the retirement ladder that closes a surface.

use super::prepared_engine_canvas::STAGED_ENGINE_SCENES;
use crate::element_contracts::{ScenePointerTarget, WorkerCell};
use flow::FlowHost;
use framework_editor::EditorHost;
#[cfg(target_arch = "wasm32")]
use js_sys;
use infinite_world::world::{tool_run_trace, WorldAssetFault};
use serde_json::{json, Value};
use semio_framework_os_node_graph::GraphHost;
use semio_framework_surface::paint::{PaintTarget, RasterHost};
use semio_framework_surface::tiled_map::MapHost;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::ManuallyDrop;
use std::sync::Arc;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use vello::wgpu;

#[path = "🆔️surface-ids-and-registry/🦀️.rs"]
mod surface_ids_and_registry;
pub use surface_ids_and_registry::{
    retire_string_page, EngineSurface, EngineSurfaceId, EngineSurfaceIdentity,
    EngineSurfaceLiveFreshness, EngineSurfaceRegistry, EngineSurfaceSlot, EngineSurfaceSnapshot,
    EngineSurfaceToken, NodeGraphEngine, NodeGraphSyncCache, ENGINE_SURFACE_CAPACITY,
    ENGINE_SURFACE_ID_BYTE_CAPACITY,
};
use surface_ids_and_registry::retire_bytes_page;

#[path = "🧯️surface-retirement-ladder/🦀️.rs"]
mod surface_retirement_ladder;
use surface_retirement_ladder::EngineSurfaceRetirement;

#[path = "🗃️sync-caches-and-surface-helpers/🦀️.rs"]
mod sync_caches_and_surface_helpers;
pub use sync_caches_and_surface_helpers::{
    begin_engine_surface_close_token, close_engine_surface_step, empty_engine_surface,
    engine_surface_terminal_nonopaque_is_empty, engine_surface_token, engine_surface_token_at,
    engine_surface_wire_id, ensure_engine_surface, BoardSyncCache, EditorSyncCache, MapSyncCache,
    RasterSyncCache, ENGINE_SURFACES, MAP_TILE_ASSET_FAULT,
};
use sync_caches_and_surface_helpers::{
    board_sync_terminal, editor_sync_terminal, map_sync_terminal, node_graph_sync_terminal,
    raster_sync_terminal,
};
pub(super) use sync_caches_and_surface_helpers::engine_surface_live_freshness;

#[path = "✍️text-editor-delivery-and-diagnostics/🦀️.rs"]
mod text_editor_delivery_and_diagnostics;
pub use text_editor_delivery_and_diagnostics::{
    engine_camera_from_json, engine_canvas_debug_log, engine_canvas_fault_log,
    published_graph_interaction, PublishedGraphInteraction, TextEditorDeliveryInFlight,
    TextEditorDeliveryPublication, TextEditorDeliverySnapshot, TextEditorDeliveryState,
    TextEditorDraftPublication, TextEditorTypingEnd, TextEditorTypingRun,
    TEXT_EDITOR_PENDING_ECHO_CAPACITY,
};
#[cfg(target_arch="wasm32")]
pub use text_editor_delivery_and_diagnostics::text_editor_now_ms;
#[cfg(not(target_arch="wasm32"))]
pub use text_editor_delivery_and_diagnostics::text_editor_now_ms;
