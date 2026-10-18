//! 🧪️ dag-host tests (🔬️dag-unit): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::{DagChannelDirection, DagChannelRef, DagLayoutOptions, DagSelectionDomains, DagSelectionJsonCursor, DagSelectionTextFault, DagSelectionTextGrant, DagSelectionTextStep, LayoutControl};
use semio_framework_os_infinite_board_engine::{Camera, GRID_WORLD_MEDIUM, InteractionMode};
use semio_framework_os_infinite_board_dag_model::{channel_row_bounds, computation_column_divider_x, computation_node_height, computation_node_width, DAG_COMPONENT_WIDTH, DAG_INTERACTION_NODE_CAPACITY, DagInteractionPlanFault, DagPortSide, input_port_row_hit_bounds, note_text_origin_x, note_widget_size, output_port_row_hit_bounds, preview_tree_row_layouts, select_control_bounds, slider_track_bounds, slider_widget_height, slider_widget_width, variadic_input_insert_positions, variadic_output_insert_positions};
use semio_framework_os_infinite_board_dag_graph::{dag_draw_lod_label, dag_node_overlays_json, dag_set_wheel_zoom_active, draw_lod_for_frame, entity_screen_json, focus_selection_camera, hovered_node_id, selection_union_bounds_screen_json, set_hover_channel, slider_overlay_state_json};
use semio_framework_os_infinite_board_dag_host::{begin_note_edit, DagHost, dimmed_node_ids, minimap_widget_layout, note_backspace, note_insert_text, set_dimmed};
use graph::HandleRole;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use semio_framework_canvas::{Point, Scene};
use semio_framework_canvas as canvas;
use semio_framework_os_infinite_directed_presentation::*;
use semio_framework_test_fixtures::fixture_text;

include!("🧮️cursors-projection-and-selection-laws/🦀️.rs");
include!("🎚️reorganize-sliders-and-label-overlays/🦀️.rs");
include!("📐️area-select-align-and-entity-screen-json/🦀️.rs");
include!("🔗️reconnect-proximity-and-lod-hit-laws/🦀️.rs");
include!("🔍️detail-lod-variadic-wheel-and-minimap/🦀️.rs");
include!("📝️manual-lod-notes-and-preview-tree/🦀️.rs");
