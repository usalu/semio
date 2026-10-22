//! ⚙️ Board engine: the retained graph engine with camera, selection and hit testing, plus the directed port descriptors it exposes.
//!
//! Crate `semio-framework-os-infinite-board-engine`; a downward producer of the infinite board family.

use semio_framework_os_infinite_board_schema::{apply_board_palette_overlay, apply_force_layout_resolved, apply_hierarchical_inner, board_visible_option, board_visible_or_true, BoardNodeShape, BoardPalette, BoardPaletteOverlay, BoardSnapshot, BoardVisibility, decode_redraw_options_json, field, ForceAdmission, ForceGraphLayoutOptions, HierarchicalTreeLayoutOptions, LayoutControl, LayoutError, LayoutIoError, NodeDescriptor, RedrawLayoutOptions, resolve_node_id_endpoint, Selection};
use semio_framework_canvas::{Affine, Color, CubicBez, FillRule, OpaqueSceneRetirementStep, Point, RasterImage, Rect, Scene, Stroke, Vec2};
use std::sync::Arc;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::cell::{Cell, RefCell};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use serde::{Deserialize, Serialize};
use graph::{Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, Undirected};
use semio_framework_value::{DslValue, FromValue as _, NativeEncodeControl, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_os_kernel::os_spr::causal::transition::HistoryFoldIndex;
use semio_framework_pack_json::JsonMemberPolicy;
use std::mem::ManuallyDrop;
use semio_framework_graph_manifest::PropertyBag;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetirementStep, RetireOwned, sequence, sequence_birth_bytes};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_canvas::{append_svg_document, SvgDocument};
use semio_framework_canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
use semio_framework_canvas as canvas;
use semio_framework_graph_drawing::routing::{compute_edge_bezier_outward, handle_exterior_cap_peak, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle};
use geometry::{distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
use semio_framework_graph_drawing::force;
use semio_framework_os_infinite_directed_presentation::*;

#[path = "🧷️point-retirement-and-engine-aliases/🦀️.rs"]
mod point_retirement_and_engine_aliases;
pub(crate) use point_retirement_and_engine_aliases::RetainedPoint;
pub use point_retirement_and_engine_aliases::{DirectedGraphEngine, UndirectedEdge, UndirectedGraphEngine, UndirectedPortGraphEngine};

#[path = "🧱️style-kinds-and-data-records/🦀️.rs"]
mod style_kinds_and_data_records;
pub use style_kinds_and_data_records::{
    ActiveUtility, BoardElementStyleKind, CompatSpecificity, EdgeData, EdgeKindDef, EdgeStrokePattern, GestureStage, GraphPortMode, Interaction, LinkCompatRule, NodeData, NodeKindDef, SelectionOptions, WireData, WireKindDef,
};

#[path = "🖌️canvas-palette/🦀️.rs"]
mod canvas_palette;
pub use canvas_palette::CanvasPalette;

#[path = "🎟️icon-paint-cache/🦀️.rs"]
mod icon_paint_cache;
pub use icon_paint_cache::{CachedIconBody, CachedIconPaintLease, IconPaintCache, IconPaintRetirementStep, IconScreenRect};

#[path = "📜️descriptors-and-snapshot-json/🦀️.rs"]
mod descriptors_and_snapshot_json;
pub use descriptors_and_snapshot_json::{
    decode_board_scene_json, decode_board_snapshot_json, encode_board_stroke_scene, force_snapshot_json, redraw_snapshot_json, snap_snapshot_json, BoardEngine, DirectedPortGraphEngine, Edge, EdgeDescriptor, GraphExtension, HandleData,
    HandleDescriptor, HandleKindDef, NodeKindHandleTemplate, RegionDescriptor, SceneDescriptor, WireDescriptor,
};

#[path = "📷️camera-events-and-hit-model/🦀️.rs"]
mod camera_events_and_hit_model;
pub use camera_events_and_hit_model::{distance, handle_position, BoardEvent, Camera, GraphPickTarget, GraphPortModel, InteractionMode, RenderSnapshot, DEFAULT_PROXIMITY_DISTANCE_WORLD};
use camera_events_and_hit_model::{node_contains_point, HitObject, ProximityConnection, WireEndpoints};

#[path = "🔲️selection-marquee/🦀️.rs"]
mod selection_marquee;
pub use selection_marquee::{
    area_preselect_ids, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds,
    selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, selection_screen_overlay_points, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, GRID_FACTOR_DEFAULT, GRID_WORLD_LARGE, GRID_WORLD_MEDIUM, GRID_WORLD_MICRO,
    GRID_WORLD_SMALL, SELECTION_CLICK_MAX_DISTANCE_PX, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX,
};

#[path = "🧭️transform-gumball/🦀️.rs"]
mod transform_gumball;
pub use transform_gumball::{
    rotate_point_about, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, transform_ring_hit, transform_ring_radius_world, TransformGumballFlags, TRANSFORM_RING_GAP_PX, TRANSFORM_RING_HIT_TOLERANCE_PX,
    TRANSFORM_RING_MIN_RADIUS_PX, TRANSFORM_ROTATE_SNAP_RADIANS,
};

#[path = "🗺️target-regions/🦀️.rs"]
mod target_regions;
pub use target_regions::{region_bounds, region_grip_at, region_grip_drag, snap_region_scalar, RegionData, RegionGrip, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD};

#[path = "🏗️engine-state-and-close/🦀️.rs"]
mod engine_state_and_close;
pub use engine_state_and_close::{EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep};

#[path = "🪛️engine-mutation/🦀️.rs"]
mod engine_mutation;

#[path = "🖱️engine-pointer/🦀️.rs"]
mod engine_pointer;

#[path = "🔎️engine-query-and-area-select/🦀️.rs"]
mod engine_query_and_area_select;

#[path = "🔗️engine-connection-and-hit-test/🦀️.rs"]
mod engine_connection_and_hit_test;

#[path = "📐️layout-entry-points/🦀️.rs"]
mod layout_entry_points;
pub use layout_entry_points::{apply_force_layout, apply_hierarchical_layout, apply_ported_force_layout, redraw_layout, snap_edge_handles};

// #endregion 🔖️Icons

// #endregion types

// #endregion 🔖️HandleKinds

// #endregion 🔖️SelectionMarquee
