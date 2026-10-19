//! 🧷️ Companion tests of engine-attach: the Scenes and EngineCanvas standalone laws with the node graph, paint2d and gesture laws that share their helpers.
//!
//! 🧭️ Each facade keeps its module name and test namespace here (`scenes`, `engine_canvas`): the standalone law files are included unchanged, the other laws are mounted from their place in the EngineCanvas tree.

#![cfg(test)]

#[path = "."]
mod scenes {
    use super::engine_canvas;
    use semio_framework_os_renderer_wgpu_engine_board::tiled_map;
    use semio_framework_os_renderer_wgpu_scene_feeds::query_map_feature_hits;
    use semio_framework_os_renderer_wgpu_scene_ink::{
        INK_EVENT_JSON_BYTE_CAPACITY, INK_INTERACTION_DOCUMENT_BYTE_CAPACITY, INK_RESIZE_HANDLES, InkBlockCursor, InkCameraF, InkDocumentJson, InkEventJsonPages, InkInteractionDocument, InkRawPages, collect_ink_document_block_spans, create_ink_item,
        erase_ink_stroke_points_in_item, flatten_ink_items, ink_effective_bounds, ink_hits_point, ink_item_id, ink_item_kind, ink_item_locked, ink_item_num, ink_items_at_point, ink_maybe_snap_fields, ink_resize_bounds, ink_resize_handle_screen_pos,
        ink_screen_to_world, ink_selection_bounds, ink_world_to_screen, scale_ink_item, validate_ink_value,
    };
    use semio_framework_os_renderer_wgpu_scene_state::{
        InkBoundsF, MAP_MARQUEE_THRESHOLD_PX, SceneCameraOwnerMetadata, SceneDrag, SceneDragMode, Viewport, camera_fixture_grant, camera_fixture_lease, mutate_scene_state, now_ms, scene_action, scene_state, schedule_scene_camera_dispatch,
        scroll_offset, set_scroll_offset,
    };
    use semio_framework_os_renderer_wgpu_scene_tables::{PAINT2D_NAVIGATOR_PADDING, Paint2dDocSyncJson, Paint2dFlatLayer, paint2d_navigator_fit_viewport, paint2d_navigator_overlay_rect};
    use semio_framework_os_renderer_wgpu_scene_text_vfs::{cursor_from_click, double_click_action, hit_double_click_target, line_col_at};
    use semio_framework_ui_layout::layout::ActionDescriptor;
    use semio_framework_ui_theme::geometry::Rect;
    use semio_framework_value::DslValue;
    use serde_json::{Value, json};
    use std::collections::BTreeMap;
    use semio_framework_ui_wgpu_input::InputState;

    include!("../../../🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs");

    #[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/📷️wgpu-canvas2d-camera/🦀️.rs"]
    mod canvas2d_camera_tests;
}

#[path = "."]
mod engine_canvas {
    use semio_framework_os_infinite_board_normal_focus::wheel_screen;
    use semio_framework_os_renderer_wgpu_engine_board::board2d::{board_drain_into_buffer, board_set_pointer_inside, coalesce_owned_board_events, with_board_host, with_board_host_mut};
    use semio_framework_os_renderer_wgpu_engine_board::tiled_map::{map_local_pointer, tiled_map_wheel_into};
    use semio_framework_os_renderer_wgpu_engine_graph::node_graph::{node_graph_pointer_down_into, node_graph_pointer_move_into, node_graph_pointer_up_into, node_graph_wheel_into};
    use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::ENGINE_SURFACES;
    use semio_framework_surface::tiled_map::MapHost;
    use semio_framework_ui_layout::layout::ActionDescriptor;
    use semio_framework_ui_theme::geometry::Rect;
    use serde_json::{Value, json};

    include!("../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🧵️wgpu-standalone-wrappers/🦀️.rs");
}


