//! 🕸️ Node graph interaction.
//!
//! Node graph camera, catalogue drag and drop, wheel, pointer, connection cursor, published interaction,
//! labels and overlays of an engine surface.

use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{ENGINE_SURFACES, EngineSurface, EngineSurfaceRegistry, NodeGraphEngine, PublishedGraphInteraction, engine_canvas_debug_log, engine_surface_wire_id, published_graph_interaction};
use semio_framework_os_renderer_wgpu_engine_board::paint2d::{paint2d_marquee_overlay, paint2d_navigator_overlay_rect};
use crate::element_contracts::FrameworkWidgetContext;
use flow::dag::dag_screen_to_world;
use flow::FlowHost;
use semio_framework_canvas as canvas;
use semio_framework_os_node_graph::GraphHost;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use semio_framework_ui_wgpu_widget_kit::draw_text_overlay;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{KeyAction, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_declaration::ui::{SurfaceKind, UiComponentSceneNode};
use semio_framework_os_infinite_board_dag_graph::{dag_set_wheel_zoom_active, fit_camera_to_content, hovered_node_id, hovered_node_id_ref, selection_union_bounds_screen_json};
use semio_framework_os_infinite_board_dag_host::{connection_cursor_active_at_screen, screen_geometry_census_json, screen_hit, screen_hit_json};

#[path = "🎥️scene-camera-and-drag-drop/🦀️.rs"]
mod scene_camera_and_drag_drop;
pub use scene_camera_and_drag_drop::{
    catalogue_ghost_descriptor_json, component_scene_apply_note_edit_key,
    component_scene_has_editable_caret, component_scene_set_caret_visible, flow_widget_drop_args,
    node_graph_catalogue_drop_action, node_graph_clear_all_ghost_widgets,
    node_graph_clear_wheel_zoom_active, node_graph_drag_ghost_descriptor, node_graph_fit_camera,
    node_graph_flow_widget_drop_action, node_graph_set_catalogue_json,
    node_graph_sync_flow_widget_ghost, puzzle3d_catalogue_drag_payload,
    puzzle3d_catalogue_drag_payload_json, CATALOGUE_DRAG_MIME, FLOW_WIDGET_DRAG_MIME,
};
#[cfg(any(test,feature="testkit"))]
pub use scene_camera_and_drag_drop::component_scene_begin_note_edit_fixture;
use scene_camera_and_drag_drop::node_graph_pan_gesture;

#[path = "🖱️wheel-and-pointer-into/🦀️.rs"]
mod wheel_and_pointer_into;
pub use wheel_and_pointer_into::{
    graph_projection_snapshot, node_graph_connection_cursor_active, node_graph_pointer_cancel_into,
    node_graph_pointer_down_into, node_graph_pointer_move_into, node_graph_pointer_up_into,
    node_graph_wheel_into, GraphInteractionSnapshot,
};
use wheel_and_pointer_into::{graph_hovered_handle, graph_plan_fault, NodeGraphPointerPlan};

#[path = "🔗️screen-pointer-and-commit/🦀️.rs"]
mod screen_pointer_and_commit;
use screen_pointer_and_commit::{
    commit_node_graph_pointer, graph_interaction_snapshot, mark_engine_scene_repaint,
    node_graph_gesture_is_screen_path, node_graph_screen_pointer_into, plan_node_graph_pointer,
    write_graph_edit_action,
};

#[path = "📌️interaction-dispatch/🦀️.rs"]
mod interaction_dispatch;
use interaction_dispatch::{
    graph_payload_digest, node_graph_interaction_domain, record_graph_interaction,
    world_to_screen_inner, write_graph_interaction_actions,
};
pub use interaction_dispatch::{
    graph_interaction_dispatch, log_graph_geometry_census, node_graph_domain_targets_json,
    GraphInteractionDispatch,
};

#[path = "🏷️labels-and-overlays/🦀️.rs"]
mod labels_and_overlays;
pub use labels_and_overlays::{
    paint_node_graph_labels, paint_node_graph_overlays, paint_paint2d_overlays,
};
