//! 🗃️ Dag board model: geometry, LOD, preview records, errors, edits and the host snapshot loaders.
//!
//! Crate `semio-framework-os-infinite-board-dag-model`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{DagLayoutOptions, field, layered_positions, LayoutControl, refusal, Selection};
use semio_framework_os_infinite_board_engine::{CanvasPalette, DirectedPortGraphEngine, GraphExtension};
use semio_framework_canvas::{Affine, Color, FillRule, Rect, Scene, Stroke};
use semio_framework_value::{DslValue, Number};
use std::collections::{HashMap, HashSet};
use semio_framework_canvas::lod::{Lod, LodScale};
use geometry::geom_sel::WorldBox;
use graph::NodeId;
use semio_framework_value::numeric_scratch::{NumericIndex, NumericSet};
use geometry::Point;
use semio_framework_value::retirement::{deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence_birth_bytes};
use semio_framework_pack_json::Value;
use ::{graph, semio_framework_canvas as canvas};
use semio_framework_graph_drawing::routing::{handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, rectangle_handle_angle_toward};
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_value::retirement::*;

#[path = "🪪️input-displaced-and-identity-records/🦀️.rs"]
mod input_displaced_and_identity_records;
pub use input_displaced_and_identity_records::{DagInputDisplaced, Identity, IdentityStep};

#[path = "🖍️labels-and-paint-helpers/🦀️.rs"]
mod labels_and_paint_helpers;
pub use labels_and_paint_helpers::{
    handle_cap_fill_path, handle_cap_peak, handle_cap_stroke_path, label_overlay_rows_for_node, node_caption_delegated_to_js_overlay, node_label_text, paint_cluster_affordances, paint_computation_column_divider, paint_computation_node_name,
    paint_computing_border_arc, paint_eval_status_border, paint_io_widget_channel_borders, paint_io_widget_name, paint_node_name_horizontal, paint_note_caret_bar, paint_slider_name, paint_variadic_plus_controls, should_paint_node_lod_icon,
};

#[path = "🧭️grid-step-port-keys-and-tree-toggle/🦀️.rs"]
mod grid_step_port_keys_and_tree_toggle;
pub use grid_step_port_keys_and_tree_toggle::{dag_port_endpoint_parts, dag_port_handle_key, dag_stroke_world_step_grid, parse_snapshot_edge_numeric_id, toggle_preview_tree_path, DagBoardEngine};

#[path = "⚠️dag-error-and-port-side/🦀️.rs"]
mod dag_error_and_port_side;
pub use dag_error_and_port_side::{DagError, DagPortSide, DAG_COMPUTATION_HEADER_ROWS};

#[path = "📏️node-sizes-and-port-rows/🦀️.rs"]
mod node_sizes_and_port_rows;
pub use node_sizes_and_port_rows::{
    channel_row_bounds, computation_io_row_count, computation_node_height, computation_node_width, input_port_connector_bounds, input_port_row_hit_bounds, io_port_column_width, io_widget_height, io_widget_width, output_port_connector_bounds,
    output_port_row_hit_bounds, port_value_types, slider_widget_height, slider_widget_width, ComputationChannelRowSide, DAG_COMPONENT_WIDTH, DAG_DISTRIBUTE_MIN_GAP, DAG_IO_COLUMN_WIDTH, DAG_LABEL_COMPACT_SCREEN_PX, DAG_LABEL_SCREEN_PX,
    DAG_SLIDER_VALUE_LABEL_RATIO,
};
use node_sizes_and_port_rows::{channel_row_center_y, computation_port_center_y, io_widget_label_center, port_center_y, DAG_NODE_EDGE_INSET};

#[path = "🖼️preview-content-layout/🦀️.rs"]
mod preview_content_layout;
pub use preview_content_layout::{
    hit_byte_in_note_line, image_widget_size, measure_preview_content, note_text_origin_x, preview_tree_row_layouts, preview_tree_rows, preview_widget_size, truncate_label_to_fit_width, WidgetPointerKind, DAG_PREVIEW_PAD, DAG_PREVIEW_ROW_HEIGHT,
    DAG_PREVIEW_TOGGLE_WIDTH, DAG_PREVIEW_TREE_INDENT,
};
use preview_content_layout::{preview_content_node_size, preview_image_node_size, uses_computation_layout};

#[path = "🎚️cluster-slider-and-widget-bounds/🦀️.rs"]
mod cluster_slider_and_widget_bounds;
pub use cluster_slider_and_widget_bounds::{
    action_control_bounds, cluster_explode_hit, cluster_explode_hit_rect, io_node_handle_angles, note_widget_size, point_in_rect, preview_content_bounds, select_control_bounds, set_slider_value_from_x, slider_track_bounds, slider_track_center,
    DAG_CLUSTER_EXPLODE_HIT_SIZE,
};

#[path = "🧮️computation-columns-and-fit/🦀️.rs"]
mod computation_columns_and_fit;
pub use computation_columns_and_fit::{
    channel_row_divider_y, computation_channel_row_count, computation_channel_row_divider_x_span, computation_column_divider_x, computation_input_column_x_bounds, computation_io_side_row_counts, computation_io_side_row_divider_indices,
    computation_name_world_center, computation_output_column_x_bounds, fit_node_size, io_node_rect_port_angle, io_node_rect_port_angle_for_node, variadic_input_insert_positions, variadic_output_insert_positions, DAG_VARIADIC_PLUS_ZOOM_THRESHOLD,
};
use computation_columns_and_fit::{computation_input_label_x, computation_output_label_x, io_widget_name_column_bounds, port_angle_on_side};

#[path = "🔁️cycle-layout-and-extension/🦀️.rs"]
mod cycle_layout_and_extension;
pub use cycle_layout_and_extension::{apply_dag_layout, dag_lod_band_floor_zoom, dag_lod_index, would_create_cycle, DagExtension};
use cycle_layout_and_extension::{DAG_LODS, DAG_LOD_ZOOM_SHIFT};

#[path = "🎨️node-style-colors-and-lod/🦀️.rs"]
mod node_style_colors_and_lod;
use node_style_colors_and_lod::DAG_LABEL_SCREEN_INSET;
pub use node_style_colors_and_lod::{
    dag_draw_lod, dag_edge_body_stroke, dag_handle_body_fill, dag_handle_body_stroke, dag_label_compact_paint_px, dag_label_layout_px, dag_label_paint_px, dag_lod_scale_json, dag_node_body_stroke, dag_node_internal_chrome_stroke,
    dag_node_label_fill, dag_node_paint_fill, dag_node_stroke_screen_px, dag_world_stroke, DAG_BOUNDED_DRAG_HIT_PAD_PX, DAG_CHROME_STROKE_SCREEN_PX, DAG_HANDLE_WORLD_RADIUS,
};

#[path = "🔬️interaction-records-and-note-edit/🦀️.rs"]
mod interaction_records_and_note_edit;
pub use interaction_records_and_note_edit::{
    dag_bit_contains, dag_bit_set, DagDragOffset, DagGraphEdit, DagInteractionPlanFault, DagInteractionProjection, DagNodeMove, DagPointerIntent, DagPointerPhase, DagPointerPlan, DagPointerSnapshot, DagProjectionGesture, NoteEditState,
    DAG_INTERACTION_NODE_CAPACITY, DAG_INTERACTION_WORD_CAPACITY,
};

#[path = "📓️graph-edit-rows/🦀️.rs"]
mod graph_edit_rows;
pub use graph_edit_rows::{dag_drag_gesture_id, dag_graph_edit_row_strings, dag_graph_edit_rows_json, write_dag_graph_edit_rows, DagGraphEditRowSink, DagJournalRefusal, DagNodeEvalStatusKind, DAG_GRAPH_EDIT_CAPACITY};

#[path = "🗺️minimap-and-paint-chrome/🦀️.rs"]
mod minimap_and_paint_chrome;
pub use minimap_and_paint_chrome::{canvas_color_with_alpha, dag_node_world_bounds, minimap_widget_point_in_rect, minimap_widget_screen_to_world, pointer_event_now_ms, DagNodePaintChrome, MinimapWidgetLayout};
