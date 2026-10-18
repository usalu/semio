//! 🧭️ Dag board host: the composed `DagHost` owner with pointer, paint, minimap and note orchestration.
//!
//! Crate `semio-framework-os-infinite-board-dag-host`; a downward producer of the infinite board family.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure the call sites.")]

use semio_framework_os_infinite_board_schema::{BoardPaletteOverlay, channel, DagChannelView, DagInputFacts, DagNodeEvaluationStatus, DagNodeStatuses, DagOutputCandidates, DagRefusalView, DagScreenHit, DagScreenHitJson, DagSelectionDomain, DagSelectionSource, decode_board_palette_overlay_json, decode_dag_options_json, field, LayoutControl, LayoutProgress, Selection};
use semio_framework_os_infinite_board_engine::{Camera, CanvasPalette, GRID_FACTOR_DEFAULT, IconPaintCache, InteractionMode};
use semio_framework_os_infinite_board_normal_model::handle_port_shape;
use semio_framework_os_infinite_board_dag_model::{action_control_bounds, canvas_color_with_alpha, dag_bit_contains, dag_bit_set, DAG_BOUNDED_DRAG_HIT_PAD_PX, DAG_CHROME_STROKE_SCREEN_PX, dag_edge_body_stroke, dag_handle_body_fill, dag_handle_body_stroke, DAG_HANDLE_WORLD_RADIUS, DAG_INTERACTION_NODE_CAPACITY, DAG_INTERACTION_WORD_CAPACITY, DAG_LABEL_COMPACT_SCREEN_PX, dag_label_layout_px, dag_label_paint_px, dag_lod_index, dag_lod_scale_json, dag_node_body_stroke, dag_node_internal_chrome_stroke, dag_node_label_fill, dag_node_paint_fill, dag_node_stroke_screen_px, DAG_PREVIEW_PAD, DAG_PREVIEW_ROW_HEIGHT, DAG_PREVIEW_TOGGLE_WIDTH, DAG_PREVIEW_TREE_INDENT, DAG_VARIADIC_PLUS_ZOOM_THRESHOLD, dag_world_stroke, DagBoardEngine, DagDragOffset, DagError, DagGraphEdit, DagInputDisplaced, DagInteractionPlanFault, DagInteractionProjection, DagNodeEvalStatusKind, DagNodeMove, DagNodePaintChrome, DagPointerIntent, DagPointerPhase, DagPointerPlan, DagPortSide, DagProjectionGesture, handle_cap_fill_path, handle_cap_stroke_path, hit_byte_in_note_line, Identity, IdentityStep, label_overlay_rows_for_node, minimap_widget_point_in_rect, minimap_widget_screen_to_world, MinimapWidgetLayout, node_caption_delegated_to_js_overlay, node_label_text, note_text_origin_x, NoteEditState, paint_cluster_affordances, paint_computation_column_divider, paint_computation_node_name, paint_computing_border_arc, paint_eval_status_border, paint_io_widget_channel_borders, paint_io_widget_name, paint_node_name_horizontal, paint_note_caret_bar, paint_slider_name, paint_variadic_plus_controls, pointer_event_now_ms, preview_content_bounds, preview_tree_rows, select_control_bounds, set_slider_value_from_x, should_paint_node_lod_icon, toggle_preview_tree_path, truncate_label_to_fit_width, WidgetPointerKind};
use semio_framework_os_infinite_board_dag_graph::{connection_hit_world, connection_hit_world_while_wiring, dag_draw_lod_label, dag_node_overlays_json, dag_screen_to_world, dag_set_wheel_zoom_active, DagGraphState, DagViewportState, draw_lod_for_frame, entity_screen_json, entity_screen_rect, IdentityCursor, lod_uses_bounded_drag, port_pointer_handle_hit, process_engine_events, screen_to_world_point, sync_channel_row_pointer_hover, sync_connection_hit_picking_for_lod, sync_minimap_pointer_hover, sync_node_positions_from_engine, world_hits_handle, world_to_screen_point};
use semio_framework_canvas::{Affine, Circle, FillRule, Rect, Scene, Stroke};
use std::cell::{Cell, RefCell};
use semio_framework_value::retirement::controlled::ControlledRetirement;
use graph::{HandleId, NodeId};
use semio_framework_os_kernel::os_spr::causal::transition::{HistoryFoldIndex, HistoryFoldSet};
use web_sys::HtmlCanvasElement;
use semio_framework_pack_json::{JsonMemberPolicy, Value};
use semio_framework_value::{NativeDecodeControl, ValueError};
use semio_framework_value::numeric_scratch::{NumericIndex, NumericInsertCursor, NumericSet};
use geometry::Point;
use std::rc::Rc;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, RetirementCursor, RetireOwned, sequence, sequence_birth_bytes};
use ::{graph, semio_framework_canvas as canvas};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_async::browser::future_to_promise;
use semio_framework_graph_drawing::routing::handle_outward_at_node_rim;
use semio_framework_os_infinite_directed_presentation::*;
use wasm_bindgen::prelude::*;
use semio_framework_value::retirement::*;

#[path = "🪟️wasm-session-and-input-application/🦀️.rs"]
mod wasm_session_and_input_application;
pub use wasm_session_and_input_application::DagInputApplication;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use wasm_session_and_input_application::DagSession;

#[path = "🏠️host-core-impl/🦀️.rs"]
mod host_core_impl;

#[path = "🖍️paint-state-and-palette/🦀️.rs"]
mod paint_state_and_palette;
pub use paint_state_and_palette::{dag_set_canvas_palette, label_overlay_rows_for_node_spec};

#[path = "🎨️node-visual-paint/🦀️.rs"]
mod node_visual_paint;
use node_visual_paint::paint_node_visual;

#[path = "🧱️host-and-state-impls/🦀️.rs"]
mod host_and_state_impls;

#[path = "📝️statuses-hits-and-note-editing/🦀️.rs"]
mod statuses_hits_and_note_editing;
use statuses_hits_and_note_editing::try_widget_pointer_down;
pub use statuses_hits_and_note_editing::{
    begin_note_edit, connection_cursor_active_at_screen, dimmed_node_ids, note_backspace, note_delete_forward, note_insert_text, note_move_caret, screen_hit, screen_hit_json, screen_pointer_gesture_begins_at, set_computing_progress, set_dimmed,
    set_node_statuses,
};

#[path = "🧬️host-state-records-and-sources/🦀️.rs"]
mod host_state_records_and_sources;
pub use host_state_records_and_sources::{dag_take_pending_open_instance_id, DagHost, DagMinimapState, DagNotesState, DagPaintState, DagPointerState};

#[path = "👆️host-pointer-impl/🦀️.rs"]
mod host_pointer_impl;
pub use host_pointer_impl::derive_pointer_plan;

#[path = "🧮️projection-and-geometry-census/🦀️.rs"]
mod projection_and_geometry_census;
pub use projection_and_geometry_census::screen_geometry_census_json;
use projection_and_geometry_census::{derive_projection_down, derive_projection_move};

#[path = "🗺️minimap-widget-and-retirement/🦀️.rs"]
mod minimap_widget_and_retirement;
pub(crate) use minimap_widget_and_retirement::{minimap_widget_json, minimap_widget_pointer_hit, paint_minimap_widget};
pub use minimap_widget_and_retirement::{minimap_widget_layout, DagHostRetirement};
