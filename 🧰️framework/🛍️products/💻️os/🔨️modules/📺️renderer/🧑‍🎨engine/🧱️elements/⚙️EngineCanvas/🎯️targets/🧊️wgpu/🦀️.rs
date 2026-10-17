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

use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry;
pub(crate) use engine_surface_registry::{
    begin_engine_surface_close_token, close_engine_surface_step,
    engine_surface_terminal_nonopaque_is_empty, engine_surface_token, engine_surface_token_at,
    EngineSurfaceToken, ENGINE_SURFACE_CAPACITY,
};
pub use engine_surface_registry::TextEditorTypingEnd;

use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas;
pub(crate) use prepared_engine_canvas::{
    stage_engine_packet_step, EngineCanvasBuildContext, EngineCanvasPacket, EngineCanvasPresenter,
};

use semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry;
pub use tutorial_surface_geometry::resolve_tutorial_surface_point;
pub(crate) use tutorial_surface_geometry::tutorial_polyline_point;

use semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach;
pub(crate) use engine_surface_attach::{advance_raster_composite, register_engine_surface};
pub use engine_surface_attach::{
    board2d_tool_run_trace_cursors, engine_raster_key, stage_engine_scene_paint,
    sync_board2d_scene, sync_engine_scene, sync_node_graph_scene, sync_paint2d_scene,
    sync_text_editor_scene, sync_tiled_map_scene, take_engine_surface_registrations,
    EngineSurfaceKindDetail, EngineSurfaceRegistration,
};

use semio_framework_os_renderer_wgpu_engine_graph::node_graph;
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

use semio_framework_os_renderer_wgpu_engine_board::tiled_map;
pub use tiled_map::{
    apply_map_tile_bytes, board_local_pointer, map_local_pointer, map_marquee_crossing,
    map_marquee_mode, parse_map_feature_hit, parse_map_hover,
    retire_map_interaction_owner, stamp_map_interaction_owner, take_map_tile_asset_fault,
    tiled_map_camera, tiled_map_publish_camera_into, tiled_map_set_camera_silent,
    tiled_map_wheel_into, with_map_host, with_map_host_mut, with_map_interaction_into,
};

use semio_framework_os_renderer_wgpu_engine_board::board2d;
pub use board2d::{
    board2d_presence_state, board_drag_active, board_pick_best_target_id,
    drive_board_authority_step, publish_board_event_step, publish_board_pointer_step,
    board2d_camera, board2d_hover_into, board2d_key_into,
    board2d_pointer_cancel_into, board2d_pointer_down_into,
    board2d_pointer_leave_into, board2d_pointer_move_into, board2d_pointer_up_into,
    board2d_publish_camera_into, board2d_set_camera_silent, board2d_wheel_into,
    board2d_yield_to_pinch_into, release_board_pointer_claim,
    settle_board_pointer_authority_into, with_board_host, with_board_host_mut,
    CoalescedBoardEvents,
};

use semio_framework_os_renderer_wgpu_engine_board::paint2d;
pub use paint2d::{
    board2d_context_menu_target, node_graph_context_menu_target, paint2d_context_menu_target,
    paint2d_marquee_overlay, paint2d_navigator_overlay_rect, paint2d_pointer_button_into,
    paint2d_pointer_cancel_into, paint2d_pointer_move_into, paint2d_wheel_into,
    text_editor_context_menu_target, with_raster_host_mut,
};

use semio_framework_os_renderer_wgpu_engine_text::text_editor;
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
    text_editor_wheel_into,
};

