//! 🕸️ Dag board graph and viewport components with the engine sync, selection and projection functions.
//!
//! Crate `semio-framework-os-infinite-board-dag-graph`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{channel, DagChannelDirection, DagChannelRef, DagChannelView, DagHoverFacts, DagLayoutOptions, DagSelectionDomains, DagWireTypeRefusal, LayoutControl, refusal, Selection};
use semio_framework_os_infinite_board_engine::{BoardEvent, Camera, CanvasPalette, distance, GRID_WORLD_LARGE, GRID_WORLD_MEDIUM, GRID_WORLD_MICRO, GRID_WORLD_SMALL, handle_position, InteractionMode};
use semio_framework_os_infinite_board_normal_model::handle_port_shape;
use semio_framework_os_infinite_board_dag_model::{action_control_bounds, apply_dag_layout, channel_row_bounds, channel_row_divider_y, cluster_explode_hit, computation_channel_row_count, computation_channel_row_divider_x_span, computation_input_column_x_bounds, computation_io_side_row_counts, computation_io_side_row_divider_indices, computation_output_column_x_bounds, ComputationChannelRowSide, dag_bit_contains, dag_bit_set, DAG_CHROME_STROKE_SCREEN_PX, DAG_COMPUTATION_HEADER_ROWS, DAG_DISTRIBUTE_MIN_GAP, dag_drag_gesture_id, dag_draw_lod, DAG_GRAPH_EDIT_CAPACITY, dag_handle_body_fill, dag_handle_body_stroke, DAG_HANDLE_WORLD_RADIUS, DAG_INTERACTION_NODE_CAPACITY, DAG_INTERACTION_WORD_CAPACITY, DAG_LABEL_SCREEN_PX, dag_node_internal_chrome_stroke, dag_node_world_bounds, dag_port_endpoint_parts, dag_port_handle_key, DAG_SLIDER_VALUE_LABEL_RATIO, dag_stroke_world_step_grid, DAG_VARIADIC_PLUS_ZOOM_THRESHOLD, dag_world_stroke, DagBoardEngine, DagError, DagGraphEdit, DagInteractionPlanFault, DagInteractionProjection, DagJournalRefusal, DagNodePaintChrome, DagPointerPlan, DagPortSide, DagProjectionGesture, fit_node_size, handle_cap_fill_path, handle_cap_peak, handle_cap_stroke_path, Identity, IdentityStep, input_port_connector_bounds, input_port_row_hit_bounds, io_node_rect_port_angle_for_node, output_port_connector_bounds, output_port_row_hit_bounds, paint_computation_column_divider, parse_snapshot_edge_numeric_id, point_in_rect, port_value_types, preview_tree_row_layouts, select_control_bounds, slider_track_bounds, WidgetPointerKind, would_create_cycle};
use semio_framework_canvas::{Circle, FillRule, Rect, Stroke};
use graph::{EdgeId, HandleId, HandleRole, NodeId, NodeShape};
use geometry::geom_sel::{world_box_from_points, WorldBox};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::HashMap;
use semio_framework_os_kernel::os_spr::causal::transition::HistoryFoldIndex;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use geometry::Point;
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use ::{graph, semio_framework_canvas as canvas};
use semio_framework_graph_drawing::routing::{compute_edge_sharp_sz_path, handle_outward_at_node_rim, handle_position_on_rectangle};
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_value::retirement::*;

#[path = "🪪️identity-cursor-and-channel-row-paint/🦀️.rs"]
mod identity_cursor_and_channel_row_paint;
pub use identity_cursor_and_channel_row_paint::IdentityCursor;

#[path = "🧾️viewport-paint-and-overlay-json/🦀️.rs"]
mod viewport_paint_and_overlay_json;
pub use viewport_paint_and_overlay_json::{dag_node_overlays_json, slider_overlay_state_json};

#[path = "🧲️graph-state-selection-and-engine-rebuild/🦀️.rs"]
mod graph_state_selection_and_engine_rebuild;

#[path = "📓️graph-state-journal-and-hits/🦀️.rs"]
mod graph_state_journal_and_hits;

#[path = "🔭️viewport-lod-grid-and-camera/🦀️.rs"]
mod viewport_lod_grid_and_camera;
pub use viewport_lod_grid_and_camera::{dag_draw_lod_label, dag_set_wheel_zoom_active, draw_lod_for_frame, focus_selection_camera, set_hover_channel, set_selected_channels};

#[path = "🧮️world-snap-engine-sync-and-hit-tests/🦀️.rs"]
mod world_snap_engine_sync_and_hit_tests;
pub use world_snap_engine_sync_and_hit_tests::{
    connection_hit_world, connection_hit_world_while_wiring, lod_uses_bounded_drag, port_pointer_handle_hit, process_engine_events, screen_to_world_point, sync_channel_row_pointer_hover, sync_connection_hit_picking_for_lod,
    sync_node_positions_from_engine, world_hits_handle,
};

#[path = "🧬️state-records-and-screen-conversion/🦀️.rs"]
mod state_records_and_screen_conversion;
pub use state_records_and_screen_conversion::{dag_screen_to_world, dag_world_to_screen, DagGraphState, DagViewportState};

#[path = "🎚️graph-state-interaction-impl/🦀️.rs"]
mod graph_state_interaction_impl;

#[path = "🖱️hover-rect-and-pick-targets/🦀️.rs"]
mod hover_rect_and_pick_targets;
pub use hover_rect_and_pick_targets::{dag_pick_targets_at_screen_json, entity_screen_rect, hovered_node_id, hovered_node_id_ref, sync_minimap_pointer_hover, world_to_screen_point};

#[path = "📐️content-bounds-and-align-selection/🦀️.rs"]
mod content_bounds_and_align_selection;

#[path = "🎞️camera-fit-and-entity-screen-json/🦀️.rs"]
mod camera_fit_and_entity_screen_json;
pub use camera_fit_and_entity_screen_json::{adopt_camera_or_fit, camera_content_coverage, entity_screen_json, fit_camera_to_content, refit_camera_if_content_left_view, selection_union_bounds_screen_json};
