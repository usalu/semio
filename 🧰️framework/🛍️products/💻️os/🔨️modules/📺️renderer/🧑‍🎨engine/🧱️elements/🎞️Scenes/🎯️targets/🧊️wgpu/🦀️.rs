//! 🎬️ framework/products/os/modules/renderer/engine/elements/🎞️Scenes/component.rs — wgpu render
//! implementation for the Scenes element, extracted from lib.rs's inline `pub mod scenes { ... }`
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via `#[path =
//! "../../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod scenes;` in lib.rs in place of the former
//! inline block; the module name `scenes` is unchanged, so every existing `crate::scenes::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🎬️ Native component scene hosts for canvas-2d, tables, graphs, and 3D views.
//!
//! 🪓️ A facade over the scene crates of `semio-framework-os-renderer-wgpu-*` (scene state, canvas2d, ink, feeds, tables, text and virtual file system,
//! icon host and scene render): the module name `scenes` and every `crate::scenes::...` path are unchanged because the crates' names are re-exported
//! below.

// 🏛️ generated facade: items lifted into contracts and native-platform that code outside the crates names
pub use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceCloseOwner, AdmittedSurfaceEntry, AdmittedSurfaceFault, AdmittedSurfaceMap, AdmittedSurfaceRejected, AdmittedSurfaceToken, SCENE_SURFACE_CAPACITY, SCENE_SURFACE_ID_BYTE_CAPACITY};
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_os_renderer_wgpu_native_platform::{NativeHostTemporalFormatter, install_native_host_temporal_formatter, native_host_temporal_formatter};
// 🏛️ end generated facade (lifts)

// 🏛️ generated facade: names of the carved crate that code outside it names (scene-render)
pub use semio_framework_os_renderer_wgpu_scene_render::{CANVAS_CATALOGUE_DRAG_MIME, SceneContextMenuTarget, cancel_canvas_catalogue_drag, cancel_canvas_interactions, cancel_canvas_pointer_gesture, cancel_canvas_pointer_gesture_for, cancel_scene_list_transfer, cancel_scene_list_transfer_for_pointer, cancel_scene_list_transfer_for_window, canvas_catalogue_drag_over_into, canvas_catalogue_drop_into, canvas_pointer_button_into, canvas_pointer_move_into, canvas_pointer_terminal_step, canvas_state_snapshot, canvas_wheel_into, context_menu_surface_kind_id, debug_log, passive_scene_pointer_button, passive_scene_pointer_move, passive_scene_wheel, render_component_scene_step, request_canvas_pointer_gesture_cancel_for_window, request_stale_canvas_authority_cancel, retire_canvas_window_step, scene_context_menu_target, scene_double_click_action, scene_has_bespoke_pointer_dispatch, scene_list_hit};
// 🏛️ end generated facade (scene-render)

// 🏛️ generated facade: names of the carved crate that code outside it names (scene-state)
pub use semio_framework_os_renderer_wgpu_scene_state::{BLOCK_LIST_PALETTE_DRAG_MIME, BlockListAccessibilityControl, BlockListChromeLabels, CanvasCatalogueHover, CanvasClick, CanvasFillJson, CanvasGesture, CanvasGestureKind, CanvasGestureSlots, CanvasGradientStopJson, CanvasLayer, ComponentSurfaceCloseBridge, ComponentSurfaceCloseBridgeState, ComponentSurfaceCloseRequest, ComponentSurfaceCloseToken, EventFeedAccessibilityControl, GUMBALL, GraphTimelineAccessibilityControl, GumballConfig, GumballGesture, GumballHandle, GumballMotion, GumballSlot, GumballView, HostTemporalPresentation, IconRenderChromeLabels, InkBoundsF, InkEditKind, InkEditState, MAP_MARQUEE_THRESHOLD_PX, MOVE_EPSILON, ModelToLayer, PendingRasterAuthorityClose, PendingRasterQueue, PendingRasterQueueToken, PendingRasterSurface, PendingRasterUploadCursor, PendingRasterUploadStep, RASTER_UPLOADS_PER_SURFACE_CAPACITY, RASTER_UPLOAD_BYTE_CAPACITY, RASTER_UPLOAD_KEY_BYTE_CAPACITY, SCENE_CAMERA_DISPATCH_CAPACITY, SceneCameraDispatchCursor, SceneCameraDispatchStep, SceneCameraOwnerMetadata, SceneChromeLabels, SceneDrag, SceneDragMode, SceneEngineHosts, SceneListHit, SceneListTransferRelease, SceneListTransferSession, SceneListTransferSource, SceneListTransferStart, SceneModifiers, SceneSurfaceState, TURN_EPSILON, TableButtonAccessibilityCell, TableEditableTextAccessibilityCell, TableEditableTextFocusTarget, TableStepperAccessibilityCell, TableStepperFocusTarget, TextEditorRenameDraft, TextEditorUiState, VfsAccessibilityControl, VfsAccessibilityControlKind, VfsAccessibilityFocusTarget, Viewport, VirtualFileSystemChromeLabels, WORLD_CAMERA_LEDGER, World3dAccessibilityControl, acknowledge_component_surface_close, begin_pending_raster_authority_close, bind_scene_engine_token, canvas_active_utility, close_retired_scene_surface_one, close_scene_camera_storage, component_surface_close_occupied, component_surface_close_pending, component_surface_close_terminal, draw_ink_rect_outline, install_scene_camera_authority, list_visible_row_capacity, mount_canvas_camera, mount_scene_identity, mutate_scene_state, note_world3d_live_camera, pivot_screen, publish_component_surface_close_terminal, remember_scene_theme, render_placeholder, request_cancel, request_canvas_pointer_gesture_cancel_for_host, request_component_surface_close, reserve_list_rows, retire_scene_identity, retire_tiled_map_scene_identity, scene_action, scene_camera_action, scene_camera_storage_is_empty, scene_host_retiring, scene_hovered_control_id, scene_input_theme, scene_state, scene_surface_retirement_can_admit, schedule_scene_camera_dispatch, scroll_key, scroll_offset, seed_vfs_expanded, set_scene_last_pointer_pos, set_scroll_offset, take_component_surface_close_request, take_world_camera_diagnostic, text_editor_ui, toggle_vfs_row_expanded, total, vfs_selection_for_click, write_scene_action, write_scene_action_batch};
#[cfg(not(target_arch = "wasm32"))]
pub use semio_framework_os_renderer_wgpu_scene_state::{CANVAS_CATALOGUE_HOVER, CANVAS_GESTURE, COMPONENT_SURFACE_CLOSE_BRIDGE, PENDING_RASTER_CLOSE_OWNER, PENDING_RASTER_STATE, SCENE_CAMERA_DISPATCH_DEADLINES_MS, SCENE_CAMERA_DISPATCH_FAULT, SCENE_LIST_TRANSFER, SCENE_STATE, SCENE_SURFACE_RETIREMENT, now_ms, with_component_surface_close_bridge};
#[cfg(target_arch = "wasm32")]
pub use semio_framework_os_renderer_wgpu_scene_state::{CANVAS_CATALOGUE_HOVER, CANVAS_GESTURE, COMPONENT_SURFACE_CLOSE_BRIDGE, PENDING_RASTER_CLOSE_OWNER, PENDING_RASTER_STATE, SCENE_CAMERA_DISPATCH_DEADLINES_MS, SCENE_CAMERA_DISPATCH_FAULT, SCENE_LIST_TRANSFER, SCENE_STATE, SCENE_SURFACE_RETIREMENT, now_ms, with_component_surface_close_bridge};
// 🏛️ end generated facade (scene-state)

// 🏛️ generated facade: names of the carved crate that code outside it names (scene-canvas2d)
pub use semio_framework_os_renderer_wgpu_scene_canvas2d::{canvas_apply_blend_mode, canvas_gradient_color_at, canvas_layer_should_render, draw_checkerboard, push_linear_gradient_fill, push_radial_gradient_fill, queue_canvas_image_upload, queue_canvas_image_upload_sized, queue_canvas_image_upload_with, queue_decoded_raster_upload, render_canvas_2d, render_canvas_shape_fill, screen_points};
// 🏛️ end generated facade (scene-canvas2d)
// 🏛️ generated facade: names of the carved crate that code outside it names (scene-ink)
pub use semio_framework_os_renderer_wgpu_scene_ink::{INK_EVENT_JSON_BYTE_CAPACITY, INK_INTERACTION_DOCUMENT_BYTE_CAPACITY, INK_INTERACTION_ITEM_CAPACITY, INK_RESIZE_HANDLES, InkBlockCursor, InkCameraF, InkCameraJson, InkClipboardContentKind, InkDocumentJson, InkEventJsonPages, InkInteractionDocument, InkInteractionEvent, InkInteractionJob, InkInteractionStep, InkRawPages, cancel_ink_edit, collect_ink_document_block_spans, create_ink_item, erase_ink_stroke_points_in_item, flatten_ink_items, ink_clipboard_copy_text, ink_clipboard_paste_into, ink_clipboard_target, ink_edit_active, ink_edit_blur_into, ink_edit_key_into, ink_edit_paste_into, ink_effective_bounds, ink_gesture_abort_into, ink_hits_point, ink_item_id, ink_item_kind, ink_item_locked, ink_item_num, ink_maybe_snap_fields, ink_pointer_cancel_into, ink_resize_bounds, ink_resize_handle_screen_pos, ink_screen_to_world, ink_selection_bounds, ink_wheel_into, ink_world_to_screen, render_ink_canvas, scale_ink_item, validate_ink_value};
#[cfg(test)]
pub use semio_framework_os_renderer_wgpu_scene_ink::{ink_pointer_state_is_clear};
// 🏛️ end generated facade (scene-ink)
// 🏛️ generated facade: names of the carved crate that code outside it names (scene-feeds)
pub use semio_framework_os_renderer_wgpu_scene_feeds::{EventFeedEntryJson, NodeGraphSurface, TiledMapSurface, accepted_block_list_accessibility_controls, accepted_event_feed_accessibility_controls, accepted_graph_timeline_accessibility_controls, accepted_table_button_accessibility_cells, accepted_table_editable_text_accessibility_cells, accepted_table_stepper_accessibility_cells, accepted_vfs_accessibility_controls, accepted_world3d_accessibility_controls, acknowledge_block_list_accessibility_candidates, acknowledge_event_feed_accessibility_candidates, acknowledge_graph_timeline_accessibility_candidates, acknowledge_host_temporal_candidates, acknowledge_table_button_accessibility_candidates, acknowledge_table_editable_text_accessibility_candidates, acknowledge_table_stepper_accessibility_candidates, acknowledge_vfs_accessibility_candidates, acknowledge_world3d_accessibility_candidates, discard_block_list_accessibility_candidates, discard_event_feed_accessibility_candidates, discard_graph_timeline_accessibility_candidates, discard_host_temporal_candidates, discard_table_button_accessibility_candidates, discard_table_editable_text_accessibility_candidates, discard_table_stepper_accessibility_candidates, discard_vfs_accessibility_candidates, discard_world3d_accessibility_candidates, event_feed_accessibility_activate, event_feed_hit, event_feed_layout, event_feed_row_height, graph_timeline_accessibility_activate, graph_timeline_accessibility_controls, graph_timeline_hit, graph_timeline_scene_label_track, host_temporal_now_ms, host_temporal_reply, pinch_camera, query_map_feature_hits, render_event_feed, render_graph_timeline, seal_block_list_accessibility_candidates, seal_event_feed_accessibility_candidates, seal_graph_timeline_accessibility_candidates, seal_host_temporal_candidates, seal_table_button_accessibility_candidates, seal_table_editable_text_accessibility_candidates, seal_table_stepper_accessibility_candidates, seal_vfs_accessibility_candidates, seal_world3d_accessibility_candidates, stage_block_list_accessibility_controls, stage_event_feed_accessibility_controls, stage_graph_timeline_accessibility_controls, stage_table_button_accessibility_cells, stage_table_editable_text_accessibility_cells, stage_table_stepper_accessibility_cells, stage_vfs_accessibility_controls, stage_world3d_accessibility_controls, tiled_map_context_menu_target, tiled_map_drag_active, tiled_map_pointer_cancel_into, tiled_map_pointer_down_into, tiled_map_pointer_leave_into, tiled_map_pointer_move_into, tiled_map_pointer_up_into, tiled_map_touch_pointer_down, tiled_map_touch_pointer_move, tiled_map_touch_pointer_up_into, touch_pointer_down, touch_pointer_move, touch_pointer_up, vfs_accessibility_control_survives_candidate};
// 🏛️ end generated facade (scene-feeds)
// 🏛️ generated facade: names of the carved crate that code outside it names (scene-tables)
pub use semio_framework_os_renderer_wgpu_scene_tables::{TableColumn, TableEditableTextCommitOutcome, TableStepperKeyOutcome, block_list_accessibility_activate, block_list_hit, block_list_transfer_drop_action, block_list_transfer_start, merge_action_args, render_block_list, render_diff_view, render_table, table_button_accessibility_activate, table_button_accessibility_cells, table_editable_text_accessibility_cells, table_editable_text_commit_owned, table_editable_text_focus_target, table_hit, table_metrics, table_row_id, table_stepper_accessibility_cells, table_stepper_apply_key, table_stepper_focus_target, table_transfer_start};
#[cfg(test)]
pub use semio_framework_os_renderer_wgpu_scene_tables::{PAINT2D_NAVIGATOR_PADDING, Paint2dDocSyncJson, Paint2dFlatLayer};
// 🏛️ end generated facade (scene-tables)
// 🏛️ generated facade: names of the carved crate that code outside it names (scene-text-vfs)
pub use semio_framework_os_renderer_wgpu_scene_text_vfs::{Board2dSurface, TEXT_EDITOR_OVERLAY_ITEMS, VfsAccessibilityKeyOutcome, board2d_drag_active, board2d_pointer_down_into, board2d_pointer_leave_into, board2d_pointer_move_into, board2d_pointer_up_into, board2d_touch_pointer_down_into, board2d_touch_pointer_move, board2d_touch_pointer_up_into, board2d_wheel_into, build_vfs_visible_rows, double_click_action, hit_double_click_target, render_text_editor_overlays, render_vfs, text_editor_open_completions, text_editor_popup_key, text_editor_popup_pointer, text_editor_queue_menu_action, vfs_accessibility_activate, vfs_accessibility_apply_key, vfs_accessibility_focus_target, vfs_hit, vfs_metrics, vfs_row_id};
// 🏛️ end generated facade (scene-text-vfs)

// 🏛️ generated facade: names of the carved crate that code outside it names (icon-host)
pub use semio_framework_os_renderer_wgpu_icon_host::{ICON_RASTER_STEP_BUDGET, IconAtlasRebuild, IconAtlasRebuildStep, build_icon_atlas_scaled, icon_atlas_source_count, icon_export, render_icon_render};
// 🏛️ end generated facade (icon-host)

#[cfg(test)]
/// 🔎️ The whole Scenes module as the one text the unsplit file held: this file and every child, for the source-text laws.
pub(crate) const SCENES_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🗃️scene-state/🦀️.rs"),
    include_str!("🪣️pending-raster/🦀️.rs"),
    include_str!("📷️scene-camera-dispatch/🦀️.rs"),
    include_str!("⌨️scene-input/🦀️.rs"),
    include_str!("🎨️scene-render-entry/🦀️.rs"),
    include_str!("🧮️table-and-paint2d/🦀️.rs"),
    include_str!("🧱️block-list-and-diff/🦀️.rs"),
    include_str!("📰️event-feed-and-timeline/🦀️.rs"),
    include_str!("🖌️canvas2d/🦀️.rs"),
    include_str!("✒️ink-canvas-model-state/🦀️.rs"),
    include_str!("🖋️ink-canvas-render/🦀️.rs"),
    include_str!("🧭️node-graph-and-tiled-map/🦀️.rs"),
    include_str!("🖼️icon-render/🦀️.rs"),
    include_str!("🧩️board2d-and-vfs/🦀️.rs"),
    include_str!("✍️text-editor/🦀️.rs")
);

#[cfg(test)]
use serde_json::{json, Value};
#[cfg(test)]
use ui_wgpu::wgpu::{ActionDescriptor, Rect};

#[cfg(test)]
include!("../../🧪️tests/🧊️wgpu-standalone/🦀️.rs");
