//! 🧭️ Normal board host: the composed `BoardHost` owner with pointer and render orchestration.
//!
//! Crate `semio-framework-os-infinite-board-normal-host`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, builtin_edge_tips, closed};
use semio_framework_os_infinite_board_engine::{ActiveUtility, Camera, CanvasPalette, distance, EdgeData, GestureStage, GraphPortMode, GRID_FACTOR_DEFAULT, IconPaintCache, Interaction, merge_pick_into_selection, pick_merge_mode_for_modifiers, rotate_point_about, SceneDescriptor, SELECTION_CLICK_MAX_DISTANCE_PX, selection_drag_enclosing, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SelectionOptions, transform_pivot_of, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, transform_ring_radius_world, TransformGumballFlags};
use semio_framework_os_infinite_board_normal_model::{BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_KEY_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardEventKind, BoardEventQueue, BoardOwnedEvent, BoardPayloadBuilder, BoardPointerSpan, BoardTransformDrag, brush_preview_snapshot_from_session_json, BrushCandidatePage, DEFAULT_BRUSH_NODE_SIZE, DEFAULT_SUGGESTION_OFFSET, drag_gesture_record, gesture_release_events, LINK_DRAG_MIN_DISTANCE_PX, NodeHandlePaintLayer, NormalPortError, REGION_DEFAULT_BRUSH_EXTENT_WORLD, StyleChromePass, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{board_draw_lod_for_frame, board_json_string_bytes, BoardDeleteKind, BoardDeletePlan, BoardDeletePlanningOperation, BoardDeletePlanningPhase, BoardPointerPlan, BoardPointerPlanFault, BoardPointerPlanKind, BoardSceneState, BoardStyleState, brush_compatible_candidates, drag_delta, handle_eligible_link_target_ring, link_compatible_handle_count_on_node, link_compatible_handle_ids_on_node, link_drag_compatible_target_node_ids, link_drag_ring_target_node_id, link_snap_commit_proximity_ok, nearest_link_snap_handle_world, node_drag_proximity_handle_pair, node_has_any_free_link_compatible_handle, node_sole_free_link_compatible_handle, planned_link_edge_id, resolve_node_hit_world};
use semio_framework_os_infinite_board_normal_focus::{active_link_wire_curve, append_edges_wires_and_link, append_indirect_handle_ring, append_nodes_and_handles_with_overlay_chrome, bake_node_icon_spans, board_apply_area_preselect, board_set_camera, BoardFocusState, BoardOutboxState, BoardPointerIntent, BoardPointerPhase, cancel_node_drag, clear_selection_on_background_click, commit_area_select_from_initial, indirect_ring_node_id, interaction_overlay_entity_ids, resolve_area_selection_with_initial, resolve_hit_world, resolve_hover_world, resolve_overview_free_link_handle_pointer_world, set_hovered_id, set_selection_ids_gestured, stage_selection_ids, sync_descriptor_with, sync_link_gesture_events, sync_selection_flags_to_objects, transform_selection_node_ids, try_begin_bounded_selection_drag_at, try_commit_link_edge};
use semio_framework_os_infinite_board_normal_gestures::{append_brush_preview_paint, append_drop_preview_paint, append_target_region_paint, begin_region_paint, BoardAuthorityStep, BoardBrushState, BoardClosingState, BoardCommitState, BoardEntityRetirement, BoardGesturesState, BoardHostClosePhase, BoardPointerCommitOperation, BoardPointerPlanRetirement, BoardPointerPublication, BoardRemovedEntity, brush_build_preview_with_offset, brush_clear_slot, brush_commit_preview, brush_enter_slot, brush_finish_slot, brush_nearest_slot_source, brush_pointer_move, brush_rebuild_preview, brush_sync_preview_events, brush_update_alt, cancel_region_drag, cancel_region_paint, cancel_transform_drag, commit_region_drag, commit_region_paint, commit_transform_drag, event_authority_terminal_is_empty, pointer_lane_is_direct, quarantine_world_content_step, resolve_pick_targets_world, step_delete_planning, try_begin_region_drag_at, update_region_drag, update_region_paint, update_transform_drag};
use semio_framework_os_infinite_board_normal_fill::NEUTRAL_BOARD_NAMING;
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::cell::{Cell, RefCell};
use std::mem::ManuallyDrop;
use semio_framework_graph_manifest::PropertyBag;
use geometry::distance_between;

#[path = "🎨️brush-session-and-event-authority/🦀️.rs"]
mod brush_session_and_event_authority;

#[path = "🪝️pointer-commit-steps/🦀️.rs"]
mod pointer_commit_steps;
pub(crate) use pointer_commit_steps::step_pointer_commit_operation;

#[path = "🧹️host-default-and-close/🦀️.rs"]
mod host_default_and_close;

#[path = "🖱️pointer-screen-events/🦀️.rs"]
mod pointer_screen_events;

#[path = "🛑️cancel-and-transform-gumball/🦀️.rs"]
mod cancel_and_transform_gumball;

#[cfg(any(test, feature = "testkit"))]
#[path = "🕰️legacy-pointer-differential/🦀️.rs"]
mod legacy_pointer_differential;

#[path = "🏠️host-struct-and-retirement/🦀️.rs"]
mod host_struct_and_retirement;
pub(crate) use host_struct_and_retirement::board_node_move_owned_bytes;
pub use host_struct_and_retirement::{BoardHost, BoardHostRetirement};

#[path = "🌐️world-content-and-pointer-planning/🦀️.rs"]
mod world_content_and_pointer_planning;

#[path = "🗑️delete-planning/🦀️.rs"]
mod delete_planning;
pub use delete_planning::board_delete_selection;
#[cfg(any(test, feature = "testkit"))]
pub use delete_planning::plan_delete_selection;
pub(crate) use delete_planning::step_pending_delete_operation;

#[path = "🗺️pointer-planners/🦀️.rs"]
mod pointer_planners;
pub use pointer_planners::{begin_pointer_cancel, begin_pointer_commit};
use pointer_planners::{plan_brush_pointer, plan_drag_pointer, plan_hover_pointer, plan_link_clear_pointer, plan_link_finish_pointer, plan_link_move_pointer, plan_link_retain_pointer, plan_selection_pending_pointer, plan_selection_pointer};
