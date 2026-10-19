//! 🧩️ Engine surface attach and per-kind scene sync.
//!
//! Attaching a retained scene to an engine surface and projecting its per-kind state: surface registrations,
//! the node graph, tiled map, board, raster and text editor scene syncs and the raster composite progress.

use semio_framework_os_renderer_wgpu_engine_board::board2d::board2d_granularity_by_id;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{BoardSyncCache, ENGINE_SURFACES, ENGINE_SURFACE_CAPACITY, EngineSurface, EngineSurfaceId, EngineSurfaceSnapshot, EngineSurfaceToken, MapSyncCache, NodeGraphEngine, NodeGraphSyncCache, RasterSyncCache, engine_camera_from_json, engine_canvas_debug_log, engine_canvas_fault_log, ensure_engine_surface};
use semio_framework_os_renderer_wgpu_engine_graph::node_graph::log_graph_geometry_census;
use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::{StagedEngineScene, STAGED_ENGINE_SCENES};
use semio_framework_os_renderer_wgpu_engine_text::text_editor::{synchronize_text_editor_host, text_editor_explicit_draft_settings};
use semio_framework_os_renderer_wgpu_engine_board::tiled_map::reserve_map_tile_fetches;
use crate::element_contracts::WorkerCell;
use canvas::Color;
use flow::FlowHost;
use framework_editor::EditorHost;
use infinite_world::world::tool_run_trace;
use semio_framework_canvas as canvas;
use semio_framework_os_node_graph::GraphHost;
use semio_framework_surface::paint::{PaintTarget, RasterHost};
use semio_framework_surface::tiled_map::MapHost;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_declaration::ui::{SurfaceKind, UiComponentSceneNode};
use infinite_canvas::{board_set_camera, board_set_grid_visible};
use semio_framework_os_infinite_board_dag_graph::{adopt_camera_or_fit, refit_camera_if_content_left_view, set_hover_channel};
use semio_framework_os_infinite_board_dag_host::{set_computing_progress, set_dimmed, set_node_statuses};
use semio_framework_os_infinite_board_normal_focus::{set_camera_silent, set_selection_ids_silent};
use semio_framework_os_infinite_board_normal_gestures::set_transform_flags;

#[path = "📎️registry-and-node-graph/🦀️.rs"]
mod registry_and_node_graph;
pub use registry_and_node_graph::{
    take_engine_surface_registrations, EngineSurfaceKindDetail, EngineSurfaceRegistration,
};
use registry_and_node_graph::{
    node_graph_engine_from_scene, sync_node_graph_engine, ATTACHED_SURFACES,
};

#[path = "🗺️graph-scene-and-map-sync/🦀️.rs"]
mod graph_scene_and_map_sync;
use graph_scene_and_map_sync::sync_node_graph_evaluation;
pub use graph_scene_and_map_sync::{
    register_engine_surface, sync_node_graph_scene, sync_tiled_map_scene,
};

#[path = "🎲️board-sync/🦀️.rs"]
mod board_sync;
pub use board_sync::{board2d_tool_run_trace_cursors, sync_board2d_scene, sync_board_engine};
use board_sync::append_board_tool_run_trace;
#[cfg(any(test,feature="testkit"))]
pub use board_sync::board2d_tool_run_trace_state;

#[path = "🖌️paint-editor-and-composite/🦀️.rs"]
mod paint_editor_and_composite;
pub use paint_editor_and_composite::{
    advance_raster_composite, engine_raster_key, stage_engine_scene_paint, sync_engine_scene,
    sync_paint2d_scene, sync_text_editor_scene,
};
pub(crate) use paint_editor_and_composite::RasterCompositeProgress;
