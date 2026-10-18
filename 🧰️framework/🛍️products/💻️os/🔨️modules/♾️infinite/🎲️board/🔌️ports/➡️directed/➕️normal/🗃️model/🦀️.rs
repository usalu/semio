//! 🗃️ Normal board model: events, plans, kinds, styles-neutral records and constants shared by the board host components.
//!
//! Crate `semio-framework-os-infinite-board-normal-model`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{array, board_locked_option, board_visible_option, BoardNodeShape, BoardSnapshot, EdgeTipDef, EdgeTipGeometry, field, NodeDescriptor};
use semio_framework_os_infinite_board_engine::{BoardElementStyleKind, Camera, CanvasPalette, CompatSpecificity, EdgeDescriptor, EdgeStrokePattern, GestureStage, HandleData, HandleDescriptor, LinkCompatRule, NodeKindHandleTemplate, region_bounds, RegionDescriptor, RegionGrip, SceneDescriptor};
use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Scene, Stroke, Vec2};
use std::collections::{BTreeMap, BTreeSet};
use serde::Deserialize;
use semio_framework_canvas::lod::{Lod, LodScale};
use graph::{NodeShape, Normal};
use semio_framework_graph_manifest::PropertyValue;
use semio_framework_graph_drawing::routing::{handle_position_on_circle, handle_position_on_rectangle};

//#endregion 🧵️FillJob

#[path = "📸️session-json-gestures-and-catalog-parsing/🦀️.rs"]
mod session_json_gestures_and_catalog_parsing;
pub use session_json_gestures_and_catalog_parsing::{
    board_draw_lod_label, brush_preview_snapshot_from_session_json, drag_gesture_record, gesture_release_events, parse_catalog_tip_slot, parse_compat_specificity, push_pick_target, reject_kind_catalog_row_legacy_label, sorted_selection_ids,
};

#[path = "🎨️css-color-parsing/🦀️.rs"]
mod css_color_parsing;
pub use css_color_parsing::{parse_css_color, parse_css_hex_color};

#[path = "🖌️style-resolution-and-edge-tips/🦀️.rs"]
mod style_resolution_and_edge_tips;
pub use style_resolution_and_edge_tips::{
    append_edge_tips_on_curve, chrome_pass_for_entity, edge_stroke_for_kind_pattern, edge_stroke_for_style, explicit_style_kind, lerp_color, locked_style_dim, node_fill_for_style, node_stroke_for_style, resolve_handle_stroke_color,
    wire_stroke_for_style,
};

#[path = "🔌️port-shapes-and-brush-geometry/🦀️.rs"]
mod port_shapes_and_brush_geometry;
pub use port_shapes_and_brush_geometry::{
    brush_handle_alignment_delta, brush_template_world_pos, compat_pair_matches, handle_port_shape, handle_port_shapes_compatible, link_gesture_rule_applies_kind_strings, region_paint_bounds, single_letter_port_families_compatible,
};

#[path = "⚠️port-error/🦀️.rs"]
mod port_error;
pub use port_error::NormalPortError;

#[path = "🎚️constants-lod-and-naming/🦀️.rs"]
mod constants_lod_and_naming;
pub use constants_lod_and_naming::{
    board_lod_scale_json, BoardDrawLod, BoardNaming, BoardPlacementNaming, BOARD_HANDLE_VITALS_CAP, BOARD_LOD_SCALE, BOUNDED_DRAG_HIT_PAD_PX, DEFAULT_BRUSH_NODE_SIZE, DEFAULT_SUGGESTION_OFFSET, DEFAULT_WIRE_KIND_ID, EDGE_HIT_TOLERANCE_PX,
    HANDLE_HIT_TOLERANCE_PX, INDIRECT_HANDLE_MARKER_NODE_SCALE, INDIRECT_HANDLE_RING_GAP_NODE_SCALE, LINK_COMMIT_SNAP_TIGHT_PX, LINK_DRAG_MIN_DISTANCE_PX, LINK_HANDLE_SNAP_EXTRA_PX, REGION_DEFAULT_BRUSH_EXTENT_WORLD,
};

#[path = "🎛️chrome-passes-and-brush-candidates/🦀️.rs"]
mod chrome_passes_and_brush_candidates;
pub use chrome_passes_and_brush_candidates::{BrushCandidatePage, BrushCandidateRef, BrushPreviewSnapshot, NodeHandlePaintLayer, StyleChromePass};

#[path = "🪣️fill-stage-and-drag-records/🦀️.rs"]
mod fill_stage_and_drag_records;
pub use fill_stage_and_drag_records::{BoardFillStage, BoardPickTargetJson, BoardRegionDrag, BoardRegionPaint, BoardTransformDrag, DropPreviewSnapshot};

#[path = "📣️event-kinds-and-payload-builder/🦀️.rs"]
mod event_kinds_and_payload_builder;
pub use event_kinds_and_payload_builder::{
    board_pointer_offset, BoardEventBatchReservation, BoardEventFault, BoardEventKind, BoardEventReservation, BoardGestureMotion, BoardOwnedEvent, BoardPayloadBuilder, BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_ITEM_CAPACITY,
    BOARD_EVENT_KEY_BYTE_CAPACITY, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY,
};

#[path = "📬️owned-event-impl/🦀️.rs"]
mod owned_event_impl;

#[path = "🗂️event-queue-and-world-cache/🦀️.rs"]
mod event_queue_and_world_cache;
#[cfg(any(test, feature = "testkit"))]
pub use event_queue_and_world_cache::board_property_retirement_credits;
pub use event_queue_and_world_cache::{
    board_snapshot_scene_descriptor, descriptor_admission, BoardEventQueue, BoardPointerSpan, NodeIconSpan, WorldContentCache, BOARD_DESCRIPTOR_BYTE_CAPACITY, BOARD_DESCRIPTOR_ITEM_CAPACITY, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY,
};
