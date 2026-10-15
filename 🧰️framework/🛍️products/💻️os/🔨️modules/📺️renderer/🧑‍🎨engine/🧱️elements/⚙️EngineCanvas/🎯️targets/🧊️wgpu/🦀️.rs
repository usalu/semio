//! 🎨️ framework/products/os/modules/renderer/engine/elements/⚙️EngineCanvas/component.rs — wgpu
//! render implementation for the EngineCanvas element, extracted from lib.rs's inline
//! `pub mod engine_canvas { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via `#[path = "../../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod engine_canvas;` in
//! lib.rs in place of the former inline block; the module name `engine_canvas` is unchanged, so
//! every existing `crate::engine_canvas::...` call site elsewhere in the crate keeps resolving
//! with zero other changes.
//! 🎨️ Embeds GraphHost, FlowHost, and EditorHost via vello offscreen compositing.
//!
//! 🪓️ Split into nine children, one folder each beside this file; the module name `engine_canvas` and every public path
//! `crate::engine_canvas::...` are unchanged because the children are re-exported below. The shared contracts
//! (`WorkerCell`, `FrameworkWidgetContext`, `ScenePointerTarget`) live in `crate::element_contracts`.

#[cfg(test)]
use canvas::Color;
#[cfg(test)]
use flow::FlowHost;
#[cfg(test)]
use framework_editor::EditorHost;
#[cfg(test)]
use semio_framework_canvas as canvas;
#[cfg(test)]
use semio_framework_os_node_graph::GraphHost;
#[cfg(test)]
use semio_framework_surface::tiled_map::{tiles as map_tiles, MapHost};
#[cfg(test)]
use serde_json::{json, Value};
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::mem::ManuallyDrop;
#[cfg(test)]
use ui_wgpu::wgpu::{
    ActionDescriptor, FontAtlas, KeyAction, PointerModifiers, Rect, SurfaceKind, Theme,
    UiComponentSceneNode,
};

#[path = "🗂️engine-surface-registry/🦀️.rs"]
mod engine_surface_registry;
pub(crate) use engine_surface_registry::{
    begin_engine_surface_close_token, close_engine_surface_step,
    engine_surface_terminal_nonopaque_is_empty, engine_surface_token, engine_surface_token_at,
    EngineSurfaceToken, ENGINE_SURFACE_CAPACITY,
};
#[cfg(test)]
use engine_surface_registry::{
    empty_engine_surface, engine_surface_wire_id, ensure_engine_surface, EngineSurfaceId,
    EngineSurfaceIdentity, EngineSurfaceLiveFreshness, EngineSurfaceRegistry, EngineSurfaceSlot,
    EngineSurfaceSnapshot, NodeGraphEngine, ENGINE_SURFACES, ENGINE_SURFACE_ID_BYTE_CAPACITY,
};

#[path = "📦️prepared-engine-canvas/🦀️.rs"]
mod prepared_engine_canvas;
#[cfg(test)]
use prepared_engine_canvas::{
    engine_gpu_freshness_matches, EngineCanvasPacketDestination, EngineGpuBuildPhase,
    EngineGpuCandidate, EngineGpuSlot, StagedEngineScene, StagedEngineScenes,
    ENGINE_CANVAS_FRAME_PACKET_CAPACITY, STAGED_ENGINE_SCENES,
};
pub(crate) use prepared_engine_canvas::{
    stage_engine_packet_step, EngineCanvasBuildContext, EngineCanvasPacket, EngineCanvasPresenter,
};

#[path = "🎓️tutorial-surface-geometry/🦀️.rs"]
mod tutorial_surface_geometry;
pub use tutorial_surface_geometry::resolve_tutorial_surface_point;
#[cfg(test)]
use tutorial_surface_geometry::tutorial_graph_world_to_screen;
pub(crate) use tutorial_surface_geometry::tutorial_polyline_point;

#[path = "🧩️engine-surface-attach/🦀️.rs"]
mod engine_surface_attach;
#[cfg(test)]
pub(crate) use engine_surface_attach::board2d_tool_run_trace_state;
pub(crate) use engine_surface_attach::{advance_raster_composite, register_engine_surface};
pub use engine_surface_attach::{
    board2d_tool_run_trace_cursors, engine_raster_key, stage_engine_scene_paint,
    sync_board2d_scene, sync_engine_scene, sync_node_graph_scene, sync_paint2d_scene,
    sync_text_editor_scene, sync_tiled_map_scene, take_engine_surface_registrations,
    EngineSurfaceKindDetail, EngineSurfaceRegistration,
};

#[path = "🕸️node-graph/🦀️.rs"]
mod node_graph;
#[cfg(test)]
pub(crate) use node_graph::component_scene_begin_note_edit_fixture;
pub use node_graph::{
    catalogue_ghost_descriptor_json, component_scene_apply_note_edit_key,
    component_scene_has_editable_caret, component_scene_set_caret_visible,
    node_graph_catalogue_drop_action, node_graph_clear_all_ghost_widgets,
    node_graph_clear_wheel_zoom_active, node_graph_connection_cursor_active, node_graph_fit_camera,
    node_graph_flow_widget_drop_action, node_graph_pointer_cancel_into,
    node_graph_pointer_down_into, node_graph_pointer_move_into, node_graph_pointer_up_into,
    node_graph_set_catalogue_json, node_graph_sync_flow_widget_ghost, node_graph_wheel_into,
    paint_node_graph_labels, paint_node_graph_overlays, paint_paint2d_overlays,
    puzzle3d_catalogue_drag_payload, puzzle3d_catalogue_drag_payload_json,
};
#[cfg(test)]
use node_graph::{
    graph_interaction_dispatch, graph_projection_snapshot, node_graph_domain_targets_json,
    GraphInteractionSnapshot, PublishedGraphInteraction,
};

#[path = "🗺️tiled-map/🦀️.rs"]
mod tiled_map;
#[cfg(test)]
use tiled_map::map_tile_url;
pub use tiled_map::{
    apply_map_tile_bytes, board_local_pointer, map_local_pointer, map_marquee_crossing,
    map_marquee_mode, map_merge_selection, parse_map_feature_hit, parse_map_hover,
    retire_map_interaction_owner, stamp_map_interaction_owner, take_map_tile_asset_fault,
    tiled_map_camera, tiled_map_publish_camera_into, tiled_map_set_camera_silent,
    tiled_map_wheel_into, with_map_host, with_map_host_mut, with_map_interaction_into,
};

#[path = "🎲️board2d/🦀️.rs"]
mod board2d;
pub use board2d::{
    board2d_presence_state, board_drag_active, board_pick_best_target_id,
    drive_board_authority_step, publish_board_event_step, publish_board_pointer_step,
    puzzle_board_camera, puzzle_board_hover_into, puzzle_board_key_into,
    puzzle_board_pointer_cancel_into, puzzle_board_pointer_down_into,
    puzzle_board_pointer_leave_into, puzzle_board_pointer_move_into, puzzle_board_pointer_up_into,
    puzzle_board_publish_camera_into, puzzle_board_set_camera_silent, puzzle_board_wheel_into,
    puzzle_board_yield_to_pinch_into, release_board_pointer_claim,
    settle_board_pointer_authority_into, with_board_host, with_board_host_mut,
    CoalescedBoardEvents,
};
#[cfg(test)]
use board2d::{board_drain_into_buffer, board_set_pointer_inside, coalesce_owned_board_events};

#[path = "🖌️paint2d/🦀️.rs"]
mod paint2d;
#[cfg(test)]
use paint2d::paint2d_pick_layer;
pub use paint2d::{
    board2d_context_menu_target, node_graph_context_menu_target, paint2d_context_menu_target,
    paint2d_marquee_overlay, paint2d_navigator_overlay_rect, paint2d_pointer_button_into,
    paint2d_pointer_cancel_into, paint2d_pointer_move_into, paint2d_wheel_into,
    text_editor_context_menu_target, with_raster_host_mut,
};
#[cfg(test)]
pub use paint2d::{pick_target_hits_for_test, selection_domains_for_test};

#[path = "✍️text-editor/🦀️.rs"]
mod text_editor;
#[cfg(target_arch = "wasm32")]
pub use text_editor::semio_wgpu_host_page_hidden;
pub(crate) use text_editor::{
    drive_text_editor_outbox_step, drive_text_editor_retained_action_step,
    has_pending_text_editor_outbox, settle_text_editor_action_receipt,
    text_editor_apply_explicit_draft_into, text_editor_cancel_explicit_publication,
    text_editor_discard_explicit_draft, text_editor_explicit_draft_has_conflict,
    text_editor_explicit_draft_is_dirty, text_editor_explicit_draft_settings,
    text_editor_explicit_draft_status, text_editor_move_explicit_draft_history,
    TextEditorActionOutcome, TextEditorRetainedActionStep,
};
pub use text_editor::{
    end_every_text_editor_typing, end_text_editor_typing, text_editor_accessibility_value,
    text_editor_apply_completion_into, text_editor_apply_key_into, text_editor_caret,
    text_editor_caret_screen, text_editor_is_read_only, text_editor_pointer_button_into,
    text_editor_pointer_cancel_into, text_editor_pointer_click_into, text_editor_pointer_move_into,
    text_editor_preview_rename, text_editor_replace_all_into, text_editor_replace_selection_into,
    text_editor_select_span_into, text_editor_selection_text, text_editor_set_selection_into,
    text_editor_wheel_into, TextEditorTypingEnd,
};
#[cfg(test)]
use text_editor::{
    synchronize_text_editor_host, text_editor_scene_document, TextEditorDeliverySnapshot,
    TextEditorDeliveryState, TEXT_EDITOR_SCENE_OWNED_BYTES,
};

#[cfg(test)]
/// 🔎️ The whole EngineCanvas module as the one text the unsplit file held: this file and every child, for the source-text laws.
pub(crate) const ENGINE_CANVAS_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🗂️engine-surface-registry/🦀️.rs"),
    include_str!("📦️prepared-engine-canvas/🦀️.rs"),
    include_str!("🎓️tutorial-surface-geometry/🦀️.rs"),
    include_str!("🧩️engine-surface-attach/🦀️.rs"),
    include_str!("🕸️node-graph/🦀️.rs"),
    include_str!("🗺️tiled-map/🦀️.rs"),
    include_str!("🎲️board2d/🦀️.rs"),
    include_str!("🖌️paint2d/🦀️.rs"),
    include_str!("✍️text-editor/🦀️.rs"),
);

#[cfg(test)]
include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");

#[cfg(test)]
#[path = "../../🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"]
mod engine_surface_attach_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🕸️wgpu-node-graph/🦀️.rs"]
mod node_graph_attach_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🫳️node-graph-gestures/🦀️.rs"]
mod node_graph_gesture_tests;

#[cfg(test)]
#[path = "../../🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs"]
mod paint2d_engine_tests;
