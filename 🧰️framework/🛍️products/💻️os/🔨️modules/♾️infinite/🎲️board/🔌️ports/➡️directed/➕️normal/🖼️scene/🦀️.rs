//! 🖼️ Normal board scene and style components with the scene painting, style resolution and viewport functions over them.
//!
//! Crate `semio-framework-os-infinite-board-normal-scene`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, BoardPaletteOverlay, builtin_edge_tips, EdgeTipCatalogEntry, EdgeTipDef, EdgeTipGeometry};
use semio_framework_os_infinite_board_engine::{BoardElementStyleKind, CachedIconBody, CachedIconPaintLease, Camera, CanvasPalette, CompatSpecificity, Edge, EdgeData, EdgeKindDef, EdgeStrokePattern, GraphPortMode, GRID_WORLD_LARGE, GRID_WORLD_MEDIUM, GRID_WORLD_MICRO, GRID_WORLD_SMALL, HandleData, HandleKindDef, IconPaintCache, LinkCompatRule, NodeData, NodeKindDef, NodeKindHandleTemplate, RegionData, snap_region_scalar, WireData, WireKindDef};
use semio_framework_os_infinite_board_normal_model::{board_draw_lod_label, BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, BOARD_HANDLE_VITALS_CAP, BOARD_LOD_SCALE, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, BoardDrawLod, BoardEventFault, BoardNaming, BoardOwnedEvent, BoardPayloadBuilder, BoardPlacementNaming, BoardPointerSpan, brush_handle_alignment_delta, BrushCandidatePage, compat_pair_matches, DEFAULT_WIRE_KIND_ID, DropPreviewSnapshot, HANDLE_HIT_TOLERANCE_PX, handle_port_shapes_compatible, INDIRECT_HANDLE_MARKER_NODE_SCALE, INDIRECT_HANDLE_RING_GAP_NODE_SCALE, LINK_COMMIT_SNAP_TIGHT_PX, link_gesture_rule_applies_kind_strings, LINK_HANDLE_SNAP_EXTRA_PX, node_fill_for_style, node_stroke_for_style, NodeHandlePaintLayer, NormalPortError, parse_catalog_tip_slot, parse_compat_specificity, parse_css_color, parse_css_hex_color, reject_kind_catalog_row_legacy_label, resolve_handle_stroke_color, single_letter_port_families_compatible, WorldContentCache};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_board_normal_model::board_property_retirement_credits;
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Rect, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use serde::Deserialize;
use semio_framework_graph_manifest::{GraphManifest, PropertyValue};
use graph::{Handle, Node, NodeShape};
use std::cell::RefCell;
use semio_framework_pack_json::Value;
use semio_framework_canvas::geom_sel::{cubic_bezier_axis_bounds, inflate_world_box, world_boxes_overlap, WorldBox};
use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
use geometry::{distance_between, normalize_or_zero};

#[path = "🧱️scene-and-style-state-impls/🦀️.rs"]
mod scene_and_style_state_impls;

#[path = "🖌️brush-icon-and-highlight-paint/🦀️.rs"]
mod brush_icon_and_highlight_paint;
pub use brush_icon_and_highlight_paint::{drag_delta, paint_highlighted_node_preview};

#[path = "🔘️handle-geometry/🦀️.rs"]
mod handle_geometry;
pub use handle_geometry::{effective_handle_radius, handle_cap_attachment, handle_positions_json, handle_world_pos, indirect_handle_marker_radius_world, indirect_handle_world_pos};

#[path = "🔗️link-compatibility-and-curves/🦀️.rs"]
mod link_compatibility_and_curves;
pub use link_compatibility_and_curves::{
    board_edge_curve, board_lod_uses_bounded_drag, handle_eligible_link_target_ring, link_compatible_handle_count_on_node, link_compatible_handle_ids_on_node, link_drag_compatible_target_node_ids, link_drag_ring_target_node_id,
    link_drag_wire_curve_world, lod_disables_discrete_pick, node_has_any_free_link_compatible_handle, node_sole_free_link_compatible_handle, resolve_node_hit_world, wire_curve,
};

#[path = "🔍️draw-lod-and-style-impl/🦀️.rs"]
mod draw_lod_and_style_impl;
pub use draw_lod_and_style_impl::{board_draw_lod_for_frame, overlay_paint_state_json};

#[path = "🧲️grid-snap-and-brush-candidates/🦀️.rs"]
mod grid_snap_and_brush_candidates;
pub(crate) use grid_snap_and_brush_candidates::board_snap_world_pair;
pub use grid_snap_and_brush_candidates::{board_set_grid_visible, brush_compatible_candidates, brush_handle_anchor_world, current_draw_lod, handles_link_compatible_for_drag, region_snap_step, snap_region_point};

#[path = "🧬️scene-types-and-delete-records/🦀️.rs"]
mod scene_types_and_delete_records;
pub(crate) use scene_types_and_delete_records::BoardPropertyAudit;
use scene_types_and_delete_records::{BoardDeleteEntry, BoardPointerDelta, BoardPropertyAuditFrame};
pub use scene_types_and_delete_records::{
    BoardDeleteKind, BoardDeletePlan, BoardDeletePlanningOperation, BoardDeletePlanningPhase, BoardFillCaptureFault, BoardPointerPlanFault, BoardSceneState, BoardStyleState, BoardWheelPlan, BOARD_FILL_TEXT_BYTES,
};

#[path = "🗑️delete-plan-and-property-audit/🦀️.rs"]
mod delete_plan_and_property_audit;

#[path = "👆️pointer-plan/🦀️.rs"]
mod pointer_plan;
pub use pointer_plan::{admitted_board_pointer_id, board_json_string_bytes, BoardPointerPlan, BoardPointerPlanKind};

#[path = "🧭️scene-state-impl/🦀️.rs"]
mod scene_state_impl;

#[path = "✂️culling-markers-and-snap/🦀️.rs"]
mod culling_markers_and_snap;
pub use culling_markers_and_snap::{
    append_handle_marker, board_set_wheel_zoom_active, edge_effectively_visible, edge_selectable, edge_world_bounds_for_cull, handle_world_bounds_cull, indirect_handle_world_bounds_cull, link_snap_commit_proximity_ok, nearest_link_snap_handle_world,
    node_drag_proximity_handle_pair, paint_node_icon, planned_link_edge_id,
};

// #endregion board_host
