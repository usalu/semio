//! 🧿️ Companion tests of ui-engine: engine surfaces, render plan validator, app catalogue and worker cell.
//!
//! 🧭️ Native and wasm32: the `fixture_original_turn` caller fixture of the renderer laws (a funded original caller turn over the retained job ledger), mounted from `E/🧪️tests`.

extern crate infinite_canvas as infinite_world;

#[path = "../../../🧪️tests/🎟️original-turn/🦀️.rs"]
pub mod fixture_original_turn;

#[path = "🧰️surface-fixtures/🦀️.rs"]
pub mod surface_fixtures;

#[cfg(test)]
#[path = "."]
mod laws {
    use infinite_world::world::{
        World3dSceneBridgeStep, World3dSnapshotApplyStep, World3dState, WorldDrawRebuildStep, WorldInteractionAuthorityStep, WorldInteractionIntent, begin_world3d_dynamic_retirement, enqueue_world3d_event, step_world3d_draw_rebuild,
        step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, world3d_dynamic_retirement_terminal_is_empty, world3d_interaction_front_generation,
    };
    use semio_framework_async::CancelToken;
    #[cfg(not(target_arch = "wasm32"))]
    use semio_framework_async::WorkerPool;
    use semio_framework_canvas as canvas;
    use semio_framework_canvas::Color;
    use semio_framework_job::RetainedCloneGrant;
    use semio_framework_os_infinite_board_normal_focus::{set_camera_silent, set_selection_ids_silent};
    use semio_framework_os_infinite_board_normal_gestures::{pointer_lane_in_flight, pointer_lane_is_direct};
    use semio_framework_os_infinite_board_normal_host::begin_pointer_commit;
    use semio_framework_os_renderer_wgpu_canvas2d_host::ring;
    use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, ScenePointerTarget, WorkerCell, fixture_scene_pointer_target};
    use semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach::{
        EngineSurfaceKindDetail, board2d_tool_run_trace_cursors, board2d_tool_run_trace_state, engine_raster_key, sync_board_engine, sync_tiled_map_scene, take_engine_surface_registrations,
    };
    use semio_framework_os_renderer_wgpu_engine_board::board2d::{
        board_drain_into_buffer, board_event_flush_now, board_event_transient, board_page_dispatch_rows, board_retire_pending_events, board2d_camera, board2d_granularity_by_id, board2d_pointer_down_into, board2d_pointer_move_into,
        board2d_pointer_up_into, board2d_set_camera_silent, board2d_wheel_into, coalesce_owned_board_events, publish_board_event_step, settle_board_pointer_authority_into, with_board_host, with_board_host_mut,
    };
    use semio_framework_os_renderer_wgpu_engine_board::tiled_map::{board_local_pointer, map_tile_url, retire_map_interaction_owner, tiled_map_camera, tiled_map_set_camera_silent, with_map_host};
    use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{
        BoardSyncCache, ENGINE_SURFACE_CAPACITY, ENGINE_SURFACES, EngineSurfaceId, EngineSurfaceRegistry, EngineSurfaceSlot, begin_engine_surface_close_token, close_engine_surface_step, engine_surface_token, engine_surface_wire_id,
        ensure_engine_surface,
    };
    use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::{ENGINE_CANVAS_FRAME_PACKET_CAPACITY, EngineCanvasBuildContext, EngineCanvasPacket, STAGED_ENGINE_SCENES, StagedEngineScene, StagedEngineScenes};
    use semio_framework_os_renderer_wgpu_engine_registry::{engine_surface_law_guard, with_engine_close_context};
    use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
    use semio_framework_os_renderer_wgpu_host_winit::os_host;
    use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading;
    use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
    use semio_framework_os_renderer_wgpu_scene_feeds::{
        tiled_map_drag_active, tiled_map_pointer_cancel_into, tiled_map_pointer_down_into, tiled_map_pointer_leave_into, tiled_map_pointer_move_into, tiled_map_pointer_up_into, tiled_map_touch_pointer_down, tiled_map_touch_pointer_move,
        tiled_map_touch_pointer_up_into,
    };
    use semio_framework_os_renderer_wgpu_scene_render::render_component_scene_step;
    use semio_framework_os_renderer_wgpu_scene_state::{SceneChromeLabels, SceneEngineHosts, close_retired_scene_surface_one, mount_scene_identity, retire_scene_identity, retire_tiled_map_scene_identity};
    use semio_framework_os_renderer_wgpu_scene_text_vfs::{board2d_drag_active, board2d_touch_pointer_down_into, board2d_touch_pointer_move, board2d_touch_pointer_up_into};
    use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
    use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::{
        UI_ENGINE, UI_ENGINE_LOCALE_UNRESOLVED, UiEngineCell, close_ui_document_one, install_ui_engine_locale, register_clipped_retained_hit_targets, request_ui_document_close, retained_scene_target_at, scene_pointer_target_is_live,
        ui_document_close_pending,
    };
    use semio_framework_os_renderer_wgpu_ui_engine::{retained_document_consumer, retained_ui_engine};
    use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
    use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::{
        CHROME_ACTION_ARGS_BYTES, CHROME_ACTION_CAPACITY, ChromeLedger, SHELL_CHROME_ACCESSIBILITY_WINDOW_ID, WORLD_CAMERA_LEDGER, accessibility_window_is_visible, acknowledge_presented_input, begin_accessibility_visible_documents,
        chrome_action_args, chrome_hit_row, ledger_publish_hits, ledger_publish_surfaces, ledger_push_action, note_accessibility_visible_document, note_chrome_accessibility, project_chrome_dump, publish_accessibility_visible_documents,
        seal_presented_input_candidate, world_camera_ledger_row,
    };
    use semio_framework_os_renderer_wgpu_ui_engine_dump::introspection::*;
    use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::{board2d_surface, build_board2d_dump, mesh_stats_for_scene, walk_board2d, walk_mesh_stats};
    use crate::fixture_original_turn;
    use semio_framework_surface::tiled_map::tiles as map_tiles;
    use semio_framework_ui_declaration::ui::{SurfaceKind, UiComponentSceneNode, UiNode};
    use semio_framework_ui_layout::layout::{ActionDescriptor, UiPresence};
    use semio_framework_ui_locale::{Label, Locale};
    use semio_framework_ui_text::FontAtlas;
    use semio_framework_ui_theme::Theme;
    use semio_framework_ui_theme::geometry::Rect;
    use semio_framework_value::DslValue;
    use serde_json::{Value, json};
    use std::collections::HashMap;
    use std::mem::ManuallyDrop;
    use ui_contract::{SurfaceId, UiDocumentLease, UiFixedList, UiText};
    use ui_wgpu::wgpu::{DrawList, HitKind, HitTarget, IconAtlas, InputState, PointerModifiers, UiStackNode, UiTextNode};

    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs"]
    mod engine_surface_attach_tests;
    #[path = "../../../🧱️elements/⚙️EngineCanvas/🧪️tests/🔬️wgpu-board2d-engine/🦀️.rs"]
    mod board2d_engine_tests;
    #[path = "../../../🧱️elements/🗣️Interpreter/🧵️worker-cell/🧪️tests/🦀️.rs"]
    mod worker_cell_tests;
    #[path = "../../../🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs"]
    mod introspection_tests;
}
