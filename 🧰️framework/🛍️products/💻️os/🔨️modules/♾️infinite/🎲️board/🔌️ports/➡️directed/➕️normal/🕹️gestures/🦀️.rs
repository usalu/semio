//! 🕹️ Normal board brush, gesture, commit and closing components with their gesture and commit functions.
//!
//! Crate `semio-framework-os-infinite-board-normal-gestures`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, Selection};
use semio_framework_os_infinite_board_engine::{ActiveUtility, Edge, EdgeData, HandleData, Interaction, merge_pick_into_selection, NodeData, NodeKindHandleTemplate, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, rotate_point_about, snap_region_scalar, snap_transform_angle, transform_ring_angle_delta, TransformGumballFlags, WireData};
use semio_framework_os_infinite_board_normal_model::{BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardEventKind, BoardGestureMotion, BoardOwnedEvent, BoardPayloadBuilder, BoardPickTargetJson, BoardRegionDrag, BoardRegionPaint, BoardTransformDrag, brush_template_world_pos, BrushCandidatePage, BrushCandidateRef, BrushPreviewSnapshot, DropPreviewSnapshot, EDGE_HIT_TOLERANCE_PX, gesture_release_events, HANDLE_HIT_TOLERANCE_PX, NormalPortError, push_pick_target, region_paint_bounds, WorldContentCache};
use semio_framework_os_infinite_board_normal_scene::{admitted_board_pointer_id, board_edge_curve, BoardDeleteKind, BoardDeletePlan, BoardDeletePlanningOperation, BoardDeletePlanningPhase, BoardPointerPlan, BoardSceneState, BoardStyleState, brush_compatible_candidates, brush_handle_anchor_world, current_draw_lod, edge_selectable, effective_handle_radius, handle_cap_attachment, handle_world_pos, indirect_handle_marker_radius_world, paint_highlighted_node_preview, region_snap_step, snap_region_point, wire_curve};
use semio_framework_os_infinite_board_normal_focus::{BoardFocusState, BoardOutboxState, resolve_hit_world, restore_gesture_selection, set_hovered_id, set_selection_ids_gestured, stage_selection_ids};
use semio_framework_canvas::{Affine, CubicBez, FillRule, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeSet, HashMap};
use std::cell::Cell;
use graph::{Handle, Node, NodeShape};
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_graph_drawing::routing::{compute_edge_bezier_outward, handle_exterior_cap_peak, handle_outward_at_node_rim};
use geometry::{distance_between, distance_point_to_cubic_bezier};

#[path = "🖌️brush-and-drop-preview/🦀️.rs"]
mod brush_and_drop_preview;
pub use brush_and_drop_preview::{append_brush_preview_paint, append_drop_preview_paint, brush_enter_slot, brush_pointer_move, brush_rebuild_preview, set_drop_preview_json};

#[path = "🛤️pointer-lane-and-interaction/🦀️.rs"]
mod pointer_lane_and_interaction;
pub use pointer_lane_and_interaction::{defers_descriptor_sync_from_js, event_authority_terminal_is_empty, interaction_json, pointer_lane_in_flight, pointer_lane_is_direct, resolve_pick_targets_world};

#[path = "🏁️closing-state-and-quarantine/🦀️.rs"]
mod closing_state_and_quarantine;
pub use closing_state_and_quarantine::quarantine_world_content_step;

#[path = "🎯️brush-slot-preview/🦀️.rs"]
mod brush_slot_preview;
pub(crate) use brush_slot_preview::brush_build_preview;
pub use brush_slot_preview::{brush_build_preview_with_offset, brush_clear_slot, brush_commit_preview, brush_finish_slot, brush_nearest_slot_source, brush_sync_preview_events, brush_update_alt};

#[path = "🌀️transform-drag/🦀️.rs"]
mod transform_drag;
pub use transform_drag::{cancel_transform_drag, commit_transform_drag, set_transform_flags, update_transform_drag};

#[path = "🗺️target-regions/🦀️.rs"]
mod target_regions;
pub use target_regions::{append_target_region_paint, begin_region_paint, cancel_region_drag, cancel_region_paint, commit_region_drag, commit_region_paint, target_regions_json, try_begin_region_drag_at, update_region_drag, update_region_paint};

#[path = "🧬️gesture-state-records/🦀️.rs"]
mod gesture_state_records;
pub use gesture_state_records::{BoardDeleteOperation, BoardBrushState, BoardClosingState, BoardCommitState, BoardEntityRetirement, BoardGesturesState, BoardHostClosePhase, BoardPointerCommitOperation, BoardPointerPublication, BoardRemovedEntity};
use gesture_state_records::{BoardPropertyContainer, BoardPropertyRetirement};

#[path = "🧰️commit-operation-and-retirements/🦀️.rs"]
mod commit_operation_and_retirements;
pub use commit_operation_and_retirements::{BoardAuthorityStep, BoardPointerPlanRetirement};

#[path = "🗑️delete-planning-step/🦀️.rs"]
mod delete_planning_step;
pub use delete_planning_step::step_delete_planning;
