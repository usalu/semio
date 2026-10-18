//! 🎯️ Normal board focus and outbox components: selection, hover, picking, descriptor sync and event emission.
//!
//! Crate `semio-framework-os-infinite-board-normal-focus`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::BoardSnapshot;
use semio_framework_os_infinite_board_engine::{ActiveUtility, BoardElementStyleKind, decode_board_snapshot_json, EdgeData, EdgeStrokePattern, GestureStage, GraphPortMode, HandleData, Interaction, merge_ids_into_selection, NodeData, normalize_selection_mode, RegionData, SceneDescriptor, selection_drag_enclosing, selection_drag_shape, SelectionOptions, WireData};
use semio_framework_os_infinite_board_normal_model::{append_edge_tips_on_curve, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_snapshot_scene_descriptor, BoardDrawLod, BoardEventBatchReservation, BoardEventFault, BoardEventKind, BoardEventQueue, BoardEventReservation, BoardOwnedEvent, BOUNDED_DRAG_HIT_PAD_PX, chrome_pass_for_entity, DEFAULT_WIRE_KIND_ID, descriptor_admission, EDGE_HIT_TOLERANCE_PX, edge_stroke_for_kind_pattern, edge_stroke_for_style, explicit_style_kind, gesture_release_events, HANDLE_HIT_TOLERANCE_PX, lerp_color, locked_style_dim, node_stroke_for_style, NodeHandlePaintLayer, NodeIconSpan, NormalPortError, parse_catalog_tip_slot, parse_css_color, sorted_selection_ids, StyleChromePass, wire_stroke_for_style};
use semio_framework_os_infinite_board_normal_scene::{append_handle_marker, board_edge_curve, board_json_string_bytes, board_lod_uses_bounded_drag, BoardSceneState, BoardStyleState, BoardWheelPlan, current_draw_lod, edge_effectively_visible, edge_selectable, edge_world_bounds_for_cull, effective_handle_radius, handle_eligible_link_target_ring, handle_world_bounds_cull, handle_world_pos, handles_link_compatible_for_drag, indirect_handle_marker_radius_world, indirect_handle_world_bounds_cull, indirect_handle_world_pos, link_compatible_handle_count_on_node, link_compatible_handle_ids_on_node, link_drag_compatible_target_node_ids, link_drag_ring_target_node_id, link_drag_wire_curve_world, lod_disables_discrete_pick, node_sole_free_link_compatible_handle, paint_node_icon, wire_curve};
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::NativeDecodeControl;
use graph::{NodeShape, property_bag_from_value};
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_canvas::geom_sel::{cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
use geometry::{distance_between, distance_point_to_cubic_bezier};

#[path = "🧲️focus-and-outbox-state-impls/🦀️.rs"]
mod focus_and_outbox_state_impls;

#[path = "🖍️selection-staging-and-sync/🦀️.rs"]
mod selection_staging_and_sync;
pub(crate) use selection_staging_and_sync::push_select_event;
pub use selection_staging_and_sync::{
    board_apply_area_preselect, cancel_node_drag, clear_selection_on_background_click, commit_area_select_from_initial, restore_gesture_selection, set_preselect_state_silent, set_selection_ids, set_selection_ids_gestured, set_selection_ids_silent,
    stage_selection_ids, sync_selection_flags_to_objects,
};

#[path = "🔘️indirect-ring-and-link-targets/🦀️.rs"]
mod indirect_ring_and_link_targets;
pub(crate) use indirect_ring_and_link_targets::indirect_ring_handle_eligible;
pub use indirect_ring_and_link_targets::indirect_ring_node_id;
use indirect_ring_and_link_targets::{link_drag_target_ring_hit, link_target_ring_snapshot, sole_indirect_handle_hit_idle_selected_node, sole_indirect_handle_hit_link_target};

#[path = "🔗️link-gesture-and-hit-resolution/🦀️.rs"]
mod link_gesture_and_hit_resolution;
pub use link_gesture_and_hit_resolution::{active_link_wire_curve, resolve_hit_world, resolve_hover_world, resolve_overview_free_link_handle_pointer_world, sync_link_gesture_events};

#[path = "🧭️focus-state-impls/🦀️.rs"]
mod focus_state_impls;

#[path = "🎥️camera-style-kinds-and-overlay/🦀️.rs"]
mod camera_style_kinds_and_overlay;
pub use camera_style_kinds_and_overlay::{board_set_camera, clear_external_link_preview, interaction_overlay_entity_ids, resolve_node_style_kind, set_camera_silent};
pub(crate) use camera_style_kinds_and_overlay::{resolve_edge_stroke_paint, resolve_handle_style_kind};

#[path = "🗳️selection-drag-and-area/🦀️.rs"]
mod selection_drag_and_area;
pub use selection_drag_and_area::{resolve_area_selection_with_initial, test_resolve_node_style_kind, transform_selection_node_ids, try_begin_bounded_selection_drag_at};

#[path = "🧬️focus-types-and-descriptor-sync/🦀️.rs"]
mod focus_types_and_descriptor_sync;
pub use focus_types_and_descriptor_sync::{clear_scene, load_board_snapshot, sync_descriptor_with, BoardFocusState, BoardOutboxState, BoardPointerIntent, BoardPointerPhase};

#[path = "🎨️paint-hover-and-link-commit/🦀️.rs"]
mod paint_hover_and_link_commit;
pub use paint_hover_and_link_commit::{
    append_edges_wires_and_link, append_indirect_handle_ring, append_nodes_and_handles_with_overlay_chrome, bake_node_icon_spans, commit_wheel, load_board_snapshot_json, set_hovered_id, set_hovered_kind, try_commit_link_edge, wheel_screen,
};
