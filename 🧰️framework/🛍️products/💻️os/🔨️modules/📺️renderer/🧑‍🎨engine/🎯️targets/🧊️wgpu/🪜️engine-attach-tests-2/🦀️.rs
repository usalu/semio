//! 🪜️ Companion tests of engine-attach: the Scenes and EngineCanvas standalone laws with the node graph, paint2d and gesture laws that share their helpers (part 2).
//!
//! 🧭️ Each facade keeps its module name and test namespace here (`scenes`, `engine_canvas`): the standalone law files are included unchanged, the other laws are mounted from their place in the EngineCanvas tree.

#![cfg(test)]

#[path = "."]
mod engine_canvas {
    use flow::FlowHost;
    use framework_editor::EditorHost;
    use semio_framework_canvas as canvas;
    use semio_framework_canvas::Color;
    use semio_framework_os_infinite_board_normal_focus::wheel_screen;
    use semio_framework_os_infinite_board_normal_gestures::defers_descriptor_sync_from_js;
    use semio_framework_os_node_graph::GraphHost;
    use semio_framework_os_renderer_wgpu_canvas2d_host::local;
    use semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;
    use semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::{engine_raster_key, stage_engine_scene_paint, sync_engine_scene, sync_node_graph_scene, sync_text_editor_scene};
    use semio_framework_os_renderer_wgpu_engine_board::board2d::{
        board_drain_into_buffer, board_set_pointer_inside, board2d_pointer_down_into, board2d_pointer_up_into, board2d_wheel_into, coalesce_owned_board_events, drive_board_authority_step, publish_board_pointer_step, with_board_host,
        with_board_host_mut,
    };
    use semio_framework_os_renderer_wgpu_engine_board::paint2d::{paint2d_marquee_overlay, paint2d_navigator_overlay_rect, paint2d_pick_layer, paint2d_pointer_button_into, paint2d_pointer_move_into, paint2d_wheel_into, with_raster_host_mut};
    use semio_framework_os_renderer_wgpu_engine_board::tiled_map::{map_local_pointer, tiled_map_wheel_into, with_map_host};
    use semio_framework_os_renderer_wgpu_engine_graph::node_graph;
    use semio_framework_os_renderer_wgpu_engine_graph::node_graph::{
        CATALOGUE_DRAG_MIME, FLOW_WIDGET_DRAG_MIME, GraphInteractionSnapshot, catalogue_ghost_descriptor_json, flow_widget_drop_args, graph_interaction_dispatch, graph_projection_snapshot, node_graph_catalogue_drop_action,
        node_graph_connection_cursor_active, node_graph_domain_targets_json, node_graph_drag_ghost_descriptor, node_graph_fit_camera, node_graph_pointer_down_into, node_graph_pointer_move_into, node_graph_pointer_up_into, node_graph_wheel_into,
        puzzle3d_catalogue_drag_payload_json,
    };
    use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{
        ENGINE_SURFACE_CAPACITY, ENGINE_SURFACE_ID_BYTE_CAPACITY, ENGINE_SURFACES, EngineSurfaceId, EngineSurfaceIdentity, EngineSurfaceLiveFreshness, EngineSurfaceRegistry, EngineSurfaceSnapshot, EngineSurfaceToken, NodeGraphEngine,
        PublishedGraphInteraction, TextEditorDeliverySnapshot, TextEditorDeliveryState, TextEditorTypingEnd, empty_engine_surface, engine_surface_token, engine_surface_token_at, ensure_engine_surface,
    };
    use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::{
        ENGINE_CANVAS_FRAME_PACKET_CAPACITY, EngineCanvasBuildContext, EngineCanvasPacket, EngineCanvasPacketDestination, EngineCanvasPresenter, EngineGpuBuildPhase, EngineGpuCandidate, EngineGpuSlot, engine_gpu_freshness_matches,
        stage_engine_packet_step,
    };
    use semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::{resolve_tutorial_surface_point, tutorial_graph_world_to_screen};
    use semio_framework_os_renderer_wgpu_engine_registry::{close_engine_canvas_packet_step, engine_surface_law_guard, with_engine_close_context};
    use semio_framework_os_renderer_wgpu_engine_text::text_editor::{
        TEXT_EDITOR_SCENE_OWNED_BYTES, TextEditorActionOutcome, TextEditorRetainedActionStep, drive_text_editor_outbox_step, drive_text_editor_retained_action_step, end_every_text_editor_typing, end_text_editor_typing,
        has_pending_text_editor_outbox, settle_text_editor_action_receipt, synchronize_text_editor_host, text_editor_accessibility_value, text_editor_apply_explicit_draft_into, text_editor_apply_key_into, text_editor_cancel_explicit_publication,
        text_editor_discard_explicit_draft, text_editor_explicit_draft_has_conflict, text_editor_explicit_draft_is_dirty, text_editor_explicit_draft_status, text_editor_is_read_only, text_editor_move_explicit_draft_history,
        text_editor_replace_all_into, text_editor_scene_document,
    };
    use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
    use semio_framework_os_renderer_wgpu_shell_prefs::labels;
    use semio_framework_surface::tiled_map::MapHost;
    use semio_framework_ui_declaration::ui::{SurfaceKind, UiComponentSceneNode};
    use semio_framework_ui_layout::layout::ActionDescriptor;
    use semio_framework_ui_theme::Theme;
    use semio_framework_ui_theme::geometry::Rect;
    use semio_framework_value::DslValue;
    use serde_json::{Value, json};
    use std::collections::HashMap;
    use std::mem::ManuallyDrop;
    use semio_framework_ui_wgpu_input::{InputState, KeyAction, PointerModifiers};
use semio_framework_ui_declaration::ui::UiPresence;

    include!("../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs");

    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs"]
    mod node_graph_attach_tests;
    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🖌️wgpu-paint2d-engine/🦀️.rs"]
    mod paint2d_engine_tests;
    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs"]
    mod node_graph_gesture_tests;
    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🩺️metrics-invalidation-drain/🦀️.rs"]
    mod metrics_invalidation_drain_tests;
    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-catalogue-workflow-drop/🦀️.rs"]
    mod catalogue_workflow_drop_tests;
}
