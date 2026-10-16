//! ♾️ OS infinite family glue.

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this alias only feeds the browser
// wasm-bindgen async-fn codegen in this crate's session bridges, so it is narrowed to exclude the
// WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;
extern crate semio_framework_os_kernel as dsl;

extern crate self as infinite;

//#region 🔖️KernelModuleAliases
pub use semio_framework_os_kernel::os_dsl;
pub use semio_framework_os_kernel::os_pack;
pub use semio_framework_os_kernel::os_spr;
/// 🧬️ Components still use former kernel path names (`crate::os_store` / `os_dsl` / `os_spr`).
pub use semio_framework_os_kernel::os_store;
pub use semio_framework_os_kernel::os_vcs;
//#endregion 🔖️KernelModuleAliases

#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
#[path = "../../🦀️.rs"]
mod component;
#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
pub use component::*;

#[cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
#[path = "../../🌍️world/🦀️.rs"]
pub mod world;

#[cfg(test)] use semio_framework_canvas as canvas;

//#region 🔖️BoardFacade
/// ♾️ Explicit re-exports of the board family: every name here lives in a producer crate `semio-framework-os-infinite-board-*`.
pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardSnapshot, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
pub use semio_framework_os_infinite_board_engine::{ActiveUtility, area_preselect_ids, BoardElementStyleKind, BoardEngine, BoardEvent, CachedIconBody, CachedIconPaintLease, Camera, CanvasPalette, CompatSpecificity, DEFAULT_PROXIMITY_DISTANCE_WORLD, DirectedPortGraphEngine, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, Edge, EdgeData, EdgeDescriptor, EdgeKindDef, EdgeStrokePattern, encode_board_stroke_scene, EngineSelectionOptions, GestureStage, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortMode, GraphPortModel, handle_position, HandleData, HandleDescriptor, HandleKindDef, IconPaintCache, IconPaintRetirementStep, IconScreenRect, Interaction, InteractionMode, LinkCompatRule, merge_ids_into_selection, merge_pick_into_selection, NodeData, NodeKindDef, NodeKindHandleTemplate, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionDescriptor, RegionGrip, RenderSnapshot, rotate_point_about, SceneDescriptor, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, SelectionOptions, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags, WireData, WireDescriptor, WireKindDef};
pub use semio_framework_os_infinite_board_normal_model::{BOARD_DESCRIPTOR_BYTE_CAPACITY, BOARD_DESCRIPTOR_ITEM_CAPACITY, BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_ITEM_CAPACITY, BOARD_EVENT_KEY_BYTE_CAPACITY, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, board_lod_scale_json, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_pointer_offset, BoardEventBatchReservation, BoardEventFault, BoardEventKind, BoardEventQueue, BoardEventReservation, BoardFillStage, BoardGestureMotion, BoardNaming, BoardOwnedEvent, BoardPlacementNaming, NormalPortError};
pub use semio_framework_os_infinite_board_normal_scene::{BOARD_FILL_TEXT_BYTES, BoardFillCaptureFault, BoardPointerPlan, BoardPointerPlanFault, BoardWheelPlan};
pub use semio_framework_os_infinite_board_normal_focus::{BoardPointerIntent, BoardPointerPhase};
pub use semio_framework_os_infinite_board_normal_gestures::{BoardAuthorityStep, BoardPointerPlanRetirement, BoardPointerPublication};
pub use semio_framework_os_infinite_board_normal_fill::{BOARD_FILL_HANDLE_CAPACITY, BOARD_FILL_KIND_CAPACITY, BOARD_FILL_KIND_HANDLE_CAPACITY, BOARD_FILL_NODE_CAPACITY, BOARD_FILL_PAGE_ITEMS, BOARD_FILL_PLACEMENT_CAPACITY, BOARD_FILL_RULE_CAPACITY, BoardFillCandidateEvent, BoardFillCandidateVerdict, BoardFillCaptureStep, BoardFillCheckpoint, BoardFillCommitCandidate, BoardFillCommitHandle, BoardFillCommitPlacement, BoardFillCommitShape, BoardFillIngressHandleText, BoardFillIngressKindText, BoardFillIngressRuleText, BoardFillIngressTemplateText, BoardFillJob, BoardFillPlacement, BoardFillPreview, BoardFillResult, BoardFillSnapshot, BoardFillSnapshotCapture, BoardFillSnapshotIngress, BoardFillText, NEUTRAL_BOARD_NAMING};
pub use semio_framework_os_infinite_board_normal_host::{BoardHost, BoardHostRetirement};
pub use semio_framework_os_infinite_board_dag_model::{fit_node_size, note_widget_size, preview_widget_size, would_create_cycle};
pub use semio_framework_os_infinite_board_dag_host::DagHost;
pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
pub use semio_framework_os_kernel::{os_dsl, os_pack, os_spr, os_store, os_vcs};
pub use semio_framework_canvas::camera::CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX;
pub use semio_framework_canvas::camera::CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN;
pub use semio_framework_canvas as canvas;
pub use graph;
pub mod board {
    pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
    pub use semio_framework_os_infinite_board_engine::{area_preselect_ids, BoardEvent, Camera, DEFAULT_PROXIMITY_DISTANCE_WORLD, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, encode_board_stroke_scene, EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortModel, handle_position, InteractionMode, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, RenderSnapshot, rotate_point_about, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags};
    pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
    pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
    pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
    pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
    pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
    pub use semio_framework_canvas as canvas;
    pub use graph;
    pub use crate::io as io;
    pub use crate::normal as normal;
    pub use crate::ports as ports;
    pub mod schema {
        pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor};
        pub mod dag_input {
            pub use semio_framework_os_infinite_board_schema::{DagChannelDirection, DagChannelRef, DagComputingProgress, DagHoverFacts, DagNodeEvaluationStatus, DagNodeStatuses, DagScreenHit, DagScreenHitJson, DagSelectionDomain, DagSelectionDomains, DagSelectionSource, DagWireTypeRefusal};
            pub mod output {
                pub use semio_framework_os_infinite_board_schema::{DagChannelView, DagOutputCandidates, DagOutputSource, DagRefusalView};
            }
            pub mod progress {
                pub use semio_framework_os_infinite_board_schema::DagComputingProgress;
            }
        }
        pub mod edge_tip {
            pub use semio_framework_os_infinite_board_schema::{builtin_edge_tips, EdgeTipCatalogEntry, EdgeTipDef, EdgeTipGeometry};
        }
        pub mod layout {
            pub use semio_framework_os_infinite_board_schema::{DagLayoutOptions, DagLayoutOrientation, ForceGraphLayoutOptions, HierarchicalTreeLayoutOptions, LayoutControl, LayoutError, LayoutProgress, RedrawLayoutOptions};
        }
        pub mod layout_inferences {
            pub mod force {
                pub use semio_framework_os_infinite_board_schema::{apply_force_layout_resolved, resolve_node_id_endpoint};
                pub use semio_framework_os_infinite_board_engine::{apply_force_layout, apply_ported_force_layout};
            }
            pub mod hierarchical {
                pub use semio_framework_os_infinite_board_engine::apply_hierarchical_layout;
            }
            pub mod layered {
                pub use semio_framework_os_infinite_board_schema::layered_positions;
            }
            pub mod redraw {
                pub use semio_framework_os_infinite_board_engine::{redraw_layout, snap_edge_handles};
            }
        }
        pub mod palette {
            pub use semio_framework_os_infinite_board_schema::{apply_board_palette_overlay, BoardPalette, BoardPaletteOverlay};
        }
        pub mod visibility {
            pub use semio_framework_os_infinite_board_schema::{board_locked_option, board_visible_option, board_visible_or_true, BoardVisibility};
        }
    }
}
pub mod directed {
    pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardSnapshot, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
    pub use semio_framework_os_infinite_board_engine::{ActiveUtility, area_preselect_ids, BoardElementStyleKind, BoardEngine, BoardEvent, CachedIconBody, CachedIconPaintLease, Camera, CanvasPalette, CompatSpecificity, DEFAULT_PROXIMITY_DISTANCE_WORLD, DirectedPortGraphEngine, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, Edge, EdgeData, EdgeDescriptor, EdgeKindDef, EdgeStrokePattern, encode_board_stroke_scene, EngineSelectionOptions, GestureStage, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortMode, GraphPortModel, handle_position, HandleData, HandleDescriptor, HandleKindDef, IconPaintCache, IconPaintRetirementStep, IconScreenRect, Interaction, InteractionMode, LinkCompatRule, merge_ids_into_selection, merge_pick_into_selection, NodeData, NodeKindDef, NodeKindHandleTemplate, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionDescriptor, RegionGrip, RenderSnapshot, rotate_point_about, SceneDescriptor, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, SelectionOptions, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags, WireData, WireDescriptor, WireKindDef};
    pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
    pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
    pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
    pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
    pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
    pub use semio_framework_canvas as canvas;
    pub use crate::directed as directed;
    pub use crate::directed_dag as directed_dag;
    pub use crate::directed_normal as directed_normal;
    pub use graph;
    pub use crate::io as io;
    pub use crate::normal as normal;
    pub use crate::ports as ports;
    pub use crate::schema as schema;
    pub use crate::types as types;
    pub use crate::undirected as undirected;
}
pub mod directed_dag {
    pub use semio_framework_os_infinite_board_schema::Selection;
    pub use semio_framework_os_infinite_board_engine::{CanvasPalette, DirectedPortGraphEngine, Edge, GraphExtension, InteractionMode, RenderSnapshot};
    pub use semio_framework_os_infinite_board_dag_model::{apply_dag_layout, cluster_explode_hit, cluster_explode_hit_rect, computation_io_row_count, computation_node_height, computation_node_width, DAG_CLUSTER_EXPLODE_HIT_SIZE, dag_drag_gesture_id, dag_draw_lod, DAG_GRAPH_EDIT_CAPACITY, dag_graph_edit_row_strings, dag_graph_edit_rows_json, DAG_INTERACTION_NODE_CAPACITY, dag_lod_scale_json, DagBoardEngine, DagError, DagExtension, DagGraphEdit, DagGraphEditRowSink, DagInteractionPlanFault, DagInteractionProjection, DagJournalRefusal, DagPointerIntent, DagPointerPhase, DagPointerPlan, DagPointerSnapshot, DagPortSide, fit_node_size, image_widget_size, io_node_handle_angles, io_node_rect_port_angle, io_widget_height, io_widget_width, measure_preview_content, note_widget_size, preview_widget_size, slider_track_center, slider_widget_height, slider_widget_width, would_create_cycle, write_dag_graph_edit_rows};
    pub use semio_framework_os_infinite_board_dag_graph::{dag_screen_to_world, dag_world_to_screen};
    pub use semio_framework_os_infinite_board_dag_host::{dag_take_pending_open_instance_id, DagHost, DagHostRetirement};
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use semio_framework_os_infinite_board_dag_host::DagSession;
    pub use graph::{EdgeId, Handle, HandleId, HandleRole, Node, NodeId};
    pub use semio_framework_graph_drawing::routing::{compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outward_at_node_rim};
    pub use semio_framework_canvas as canvas;
    pub use crate::directed as graph;
    pub mod input_application {
        pub use semio_framework_os_infinite_board_dag_model::DagInputDisplaced;
        pub use semio_framework_os_infinite_board_dag_host::DagInputApplication;
    }
}
pub mod directed_normal {
    pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardSnapshot, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
    pub use semio_framework_os_infinite_board_engine::{ActiveUtility, area_preselect_ids, BoardElementStyleKind, BoardEngine, BoardEvent, CachedIconBody, CachedIconPaintLease, Camera, CanvasPalette, CompatSpecificity, DEFAULT_PROXIMITY_DISTANCE_WORLD, DirectedPortGraphEngine, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, Edge, EdgeData, EdgeDescriptor, EdgeKindDef, EdgeStrokePattern, encode_board_stroke_scene, EngineSelectionOptions, GestureStage, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortMode, GraphPortModel, handle_position, HandleData, HandleDescriptor, HandleKindDef, IconPaintCache, IconPaintRetirementStep, IconScreenRect, Interaction, InteractionMode, LinkCompatRule, merge_ids_into_selection, merge_pick_into_selection, NodeData, NodeKindDef, NodeKindHandleTemplate, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionDescriptor, RegionGrip, RenderSnapshot, rotate_point_about, SceneDescriptor, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, SelectionOptions, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags, WireData, WireDescriptor, WireKindDef};
    pub use semio_framework_os_infinite_board_normal_model::{BOARD_DESCRIPTOR_BYTE_CAPACITY, BOARD_DESCRIPTOR_ITEM_CAPACITY, BOARD_EVENT_BYTE_CAPACITY, BOARD_EVENT_ITEM_CAPACITY, BOARD_EVENT_KEY_BYTE_CAPACITY, BOARD_EVENT_PAYLOAD_BYTE_CAPACITY, board_lod_scale_json, BOARD_POINTER_BYTE_CAPACITY, BOARD_POINTER_ITEM_CAPACITY, board_pointer_offset, BoardEventBatchReservation, BoardEventFault, BoardEventKind, BoardEventQueue, BoardEventReservation, BoardFillStage, BoardGestureMotion, BoardNaming, BoardOwnedEvent, BoardPlacementNaming, NormalPortError};
    pub use semio_framework_os_infinite_board_normal_scene::{BOARD_FILL_TEXT_BYTES, BoardFillCaptureFault, BoardPointerPlan, BoardPointerPlanFault, BoardWheelPlan};
    pub use semio_framework_os_infinite_board_normal_focus::{BoardPointerIntent, BoardPointerPhase};
    pub use semio_framework_os_infinite_board_normal_gestures::{BoardAuthorityStep, BoardPointerPlanRetirement, BoardPointerPublication};
    pub use semio_framework_os_infinite_board_normal_fill::{BOARD_FILL_HANDLE_CAPACITY, BOARD_FILL_KIND_CAPACITY, BOARD_FILL_KIND_HANDLE_CAPACITY, BOARD_FILL_NODE_CAPACITY, BOARD_FILL_PAGE_ITEMS, BOARD_FILL_PLACEMENT_CAPACITY, BOARD_FILL_RULE_CAPACITY, BoardFillCandidateEvent, BoardFillCandidateVerdict, BoardFillCaptureStep, BoardFillCheckpoint, BoardFillCommitCandidate, BoardFillCommitHandle, BoardFillCommitPlacement, BoardFillCommitShape, BoardFillIngressHandleText, BoardFillIngressKindText, BoardFillIngressRuleText, BoardFillIngressTemplateText, BoardFillJob, BoardFillPlacement, BoardFillPreview, BoardFillResult, BoardFillSnapshot, BoardFillSnapshotCapture, BoardFillSnapshotIngress, BoardFillText, NEUTRAL_BOARD_NAMING};
    pub use semio_framework_os_infinite_board_normal_host::{BoardHost, BoardHostRetirement};
    pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
    pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
    pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
    pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
    pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
    pub use semio_framework_canvas::camera::CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX;
    pub use semio_framework_canvas::camera::CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN;
    pub use semio_framework_canvas as canvas;
    pub use crate::directed as directed;
    pub use crate::directed_dag as directed_dag;
    pub use crate::directed_normal as directed_normal;
    pub use graph;
    pub use crate::io as io;
    pub use crate::normal as normal;
    pub use crate::ports as ports;
    pub use crate::schema as schema;
    pub use crate::types as types;
    pub use crate::undirected as undirected;
}
pub mod io {
    pub mod text {
        pub mod dag_input {
            pub use semio_framework_os_infinite_board_schema::{decode_dag_channels_json, decode_dag_hover_json, decode_dag_node_statuses_json, decode_dag_selection_json, encode_dag_channels_json, encode_dag_hover_json, encode_dag_node_ids_json, encode_dag_selection_json};
            pub mod output {
                pub use semio_framework_os_infinite_board_schema::{DagJsonSource, DagOutputKind, DagOutputPreparation, DagPreparedOutput};
            }
            pub mod progress {
                pub use semio_framework_os_infinite_board_schema::decode_dag_computing_progress_json;
            }
            pub mod retained {
                pub use semio_framework_os_infinite_board_schema::{DagInputCursor, DagInputFacts, DagInputKind};
            }
            pub mod selection {
                pub use semio_framework_os_infinite_board_schema::{DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES, DagSelectionJsonCursor, DagSelectionTextFault, DagSelectionTextGrant, DagSelectionTextStep};
            }
        }
        pub mod layout {
            pub use semio_framework_os_infinite_board_schema::{decode_dag_options_json, decode_redraw_options_json, LayoutIoError};
            pub use semio_framework_os_infinite_board_engine::{force_snapshot_json, redraw_snapshot_json, snap_snapshot_json};
        }
        pub mod palette {
            pub use semio_framework_os_infinite_board_schema::decode_board_palette_overlay_json;
        }
        pub mod snapshot {
            pub use semio_framework_os_infinite_board_engine::decode_board_snapshot_json;
        }
        pub mod snapshot_assembly {
            pub use semio_framework_os_infinite_board_normal_focus::load_board_snapshot_json;
        }
        pub mod visibility {
            pub use semio_framework_os_infinite_board_engine::decode_board_scene_json;
        }
    }
}
pub mod normal {
    pub mod directed {
        pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
        pub use semio_framework_os_infinite_board_engine::{area_preselect_ids, BoardEvent, Camera, DEFAULT_PROXIMITY_DISTANCE_WORLD, DirectedGraphEngine, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, encode_board_stroke_scene, EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortModel, handle_position, InteractionMode, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, RenderSnapshot, rotate_point_about, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags};
        pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
        pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
        pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
        pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
        pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
        pub use semio_framework_canvas as canvas;
        pub use graph;
        pub use crate::io as io;
        pub use crate::normal as normal;
        pub use crate::ports as ports;
        pub use crate::board::schema as schema;
    }
    pub mod undirected {
        pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
        pub use semio_framework_os_infinite_board_engine::{area_preselect_ids, BoardEvent, Camera, DEFAULT_PROXIMITY_DISTANCE_WORLD, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, encode_board_stroke_scene, EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortModel, handle_position, InteractionMode, merge_ids_into_selection, merge_pick_into_selection, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, RenderSnapshot, rotate_point_about, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags, UndirectedGraphEngine};
        pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
        pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
        pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
        pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
        pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
        pub use semio_framework_canvas as canvas;
        pub use graph;
        pub use crate::io as io;
        pub use crate::normal as normal;
        pub use crate::ports as ports;
        pub use crate::board::schema as schema;
    }
}
pub mod ports {
    pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
    pub use semio_framework_os_infinite_board_engine::{area_preselect_ids, BoardEvent, Camera, DEFAULT_PROXIMITY_DISTANCE_WORLD, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, encode_board_stroke_scene, EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortModel, handle_position, HandleData, HandleDescriptor, HandleKindDef, InteractionMode, merge_ids_into_selection, merge_pick_into_selection, NodeKindHandleTemplate, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, RenderSnapshot, rotate_point_about, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags};
    pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
    pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
    pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
    pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
    pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
    pub use semio_framework_canvas as canvas;
    pub use crate::directed as directed;
    pub use crate::directed_dag as directed_dag;
    pub use crate::directed_normal as directed_normal;
    pub use graph;
    pub use crate::io as io;
    pub use crate::normal as normal;
    pub use crate::ports as ports;
    pub use crate::undirected as undirected;
    pub mod schema {
        pub use semio_framework_os_infinite_board_engine::HandleDescriptor;
    }
}
pub mod schema {
    pub use semio_framework_os_infinite_board_schema::{BoardEdgeSnapshot, BoardHandleSnapshot, BoardNodeShape, BoardNodeSnapshot, BoardRegionSnapshot, BoardSnapshot, BoardSnapshotSchema, CameraDescriptor, NodeDescriptor};
    pub use semio_framework_os_infinite_board_engine::{EdgeDescriptor, HandleDescriptor, RegionDescriptor, SceneDescriptor, WireDescriptor};
    pub mod snapshot {
        pub use semio_framework_os_infinite_board_schema::{BoardEdgeSnapshot, BoardHandleSnapshot, BoardNodeShape, BoardNodeSnapshot, BoardRegionSnapshot, BoardSnapshot, BoardSnapshotSchema};
    }
}
pub mod types {
    pub use semio_framework_os_infinite_board_engine::{ActiveUtility, BoardElementStyleKind, CachedIconBody, CachedIconPaintLease, CanvasPalette, CompatSpecificity, EdgeData, EdgeKindDef, EdgeStrokePattern, GestureStage, GraphPortMode, IconPaintCache, IconPaintRetirementStep, IconScreenRect, Interaction, LinkCompatRule, NodeData, NodeKindDef, SelectionOptions, WireData, WireKindDef};
    pub use graph::NodeShape;
}
pub mod undirected {
    pub use semio_framework_os_infinite_board_schema::{board_edge_handle_ids, board_locked_option, board_visible_option, board_visible_or_true, BoardPalette, BoardPaletteOverlay, BoardVisibility, CameraDescriptor, NodeDescriptor, Selection};
    pub use semio_framework_os_infinite_board_engine::{area_preselect_ids, BoardEvent, Camera, DEFAULT_PROXIMITY_DISTANCE_WORLD, DRAW_EDGE_TYPE_REFUSAL_WORLD_TOLERANCE, Edge, encode_board_stroke_scene, EngineSelectionOptions, GraphEngine, GraphEngineBackingRetirementStep, GraphExtension, GraphPickTarget, GraphPortModel, handle_position, HandleData, HandleDescriptor, HandleKindDef, InteractionMode, merge_ids_into_selection, merge_pick_into_selection, NodeKindHandleTemplate, normalize_selection_mode, pick_merge_mode_for_modifiers, region_bounds, region_grip_at, region_grip_drag, REGION_GRIP_PX, REGION_HIT_TOLERANCE_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, RegionData, RegionGrip, RenderSnapshot, rotate_point_about, SELECTION_CLICK_MAX_DISTANCE_PX, selection_contains_edge_curve, selection_contains_handle_point, selection_contains_node_bounds, SELECTION_DRAG_DIRECTION_THRESHOLD_PX, selection_drag_enclosing, selection_drag_enclosing_rectangle, selection_drag_shape, SELECTION_LASSO_MIN_POINT_DISTANCE_PX, SELECTION_MARQUEE_DRAG_THRESHOLD_PX, selection_screen_overlay_points, snap_region_scalar, snap_transform_angle, transform_pivot_of, transform_ring_angle_delta, TRANSFORM_RING_GAP_PX, transform_ring_hit, TRANSFORM_RING_HIT_TOLERANCE_PX, TRANSFORM_RING_MIN_RADIUS_PX, transform_ring_radius_world, TRANSFORM_ROTATE_SNAP_RADIANS, TransformGumballFlags, UndirectedPortGraphEngine};
    pub use graph::{CoreEdge, Directed, Directedness, EdgeId, ElementSemantics, GraphEdge, Handle, HandleId, HandleRole, Node, NodeId, NodeShape, Normal, orient_endpoints, Ported, PortModel, property_bag_from_value, property_bag_to_value, Undirected};
    pub use semio_framework_graph_manifest::{PropertyBag, PropertyValue};
    pub use canvas::geom_sel::{inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box, world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox};
    pub use semio_framework_graph_drawing::routing::{circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, compute_edge_sharp_sz_path, handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_exterior_cap_triangle_fill_path, handle_exterior_cap_triangle_peak, handle_exterior_cap_triangle_stroke_path, handle_outside_node_clip_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};
    pub use geometry::{clamp_f64, distance_between, distance_point_to_cubic_bezier, normalize_or_zero};
    pub use semio_framework_canvas as canvas;
    pub use crate::directed as directed;
    pub use crate::directed_dag as directed_dag;
    pub use crate::directed_normal as directed_normal;
    pub use graph;
    pub use crate::io as io;
    pub use crate::normal as normal;
    pub use crate::ports as ports;
    pub use crate::ports::schema as schema;
    pub use crate::undirected as undirected;
}
pub use semio_framework_os_infinite_board_normal_scene::{board_set_grid_visible, board_set_wheel_zoom_active};
pub use semio_framework_os_infinite_board_normal_focus::board_set_camera;
pub use semio_framework_os_infinite_board_normal_host::board_delete_selection;
pub use semio_framework_os_infinite_board_dag_graph::{dag_draw_lod_label, dag_pick_targets_at_screen_json, dag_set_wheel_zoom_active};
pub use semio_framework_os_infinite_board_dag_host::dag_set_canvas_palette;
//#endregion 🔖️BoardFacade
