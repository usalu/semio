//! 🧩️ framework/products/os/modules/renderer/engine/elements/🟦️Interpreter/component.rs — wgpu
//! interpreter implementation for the Interpreter element, extracted from lib.rs's inline
//! `pub mod interpreter { ... }` body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired
//! via the renderer's exact `../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` module path in
//! lib.rs in place of the former inline block; the module name `interpreter` is unchanged, so
//! every existing `crate::interpreter::...` call site elsewhere in the crate keeps resolving
//! with zero other changes.
//! 🧩️ Maps framework UiNode trees to ui_wgpu widget nodes.
//!
//! 🪓️ Split into ten children, one folder each beside this file; the module name `interpreter` and every public path
//! `crate::interpreter::...` are unchanged because the children are re-exported below. The shared contracts
//! (`WorkerCell`, `FrameworkWidgetContext`, `ScenePointerTarget`, `RasterUploadRefusal`) live in `crate::element_contracts`.

use semio_framework_os_renderer_wgpu_ui_engine::render_plan_validator;
pub use render_plan_validator::{validate_component_scene, RenderPlanLimits, RENDER_PLAN_LIMITS};

use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
pub use retained_ui_engine::{
    active_retained_drag_sessions, cancel_scene_pointer, canvas_catalogue_drag_over,
    canvas_catalogue_drop, captured_scene_pointer, claim_scene_pointer_at,
    claim_scene_pointer_owner, dispatch_accessibility_event, dispatch_presented_ui_pointer_event,
    dispatch_ui_event, dispatch_ui_pointer_event, register_clipped_retained_hit_targets,
    register_retained_hit_targets, release_scene_pointer, retained_any_pointer_capture,
    retained_content_height, retained_node_is_select, retained_pointer_capture_window,
    retained_scene_target, retained_scene_target_at, scene_pointer_target_is_live,
};
pub(crate) use retained_ui_engine::{
    advance_retained_window_clock, close_ui_document_one, component_scene_is_presented,
    install_ui_engine_locale, request_ui_document_close, retained_node_focus_identity,
    retained_node_has_visible_focus, settle_retained_input_receipt, tree_window_measures,
    ui_document_close_pending, ui_document_close_pending_for, RetainedNodeFocusKind, UiEngineCell,
    UI_ENGINE_LOCALE_UNRESOLVED,
};

use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus;
pub(crate) use text_and_table_focus::sync_focused_table_editable_text_draft;
pub use text_and_table_focus::{
    apply_focused_node_graph_note_key, apply_focused_table_editable_text_key,
    apply_focused_table_stepper_key, apply_focused_text_editor_key, apply_focused_text_editor_text,
    blur_focused_node_graph_caret_for_pointer, blur_focused_table_editable_text_for_pointer,
    blur_focused_table_stepper_for_pointer, blur_focused_text_editor_for_pointer,
    refresh_node_graph_caret_after_pointer,
};

use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus;
pub use ink_and_vfs_focus::{
    abort_focused_ink_clipboard_stream, abort_focused_text_editor_stream,
    apply_focused_ink_editor_key, apply_focused_ink_paste, apply_focused_ink_surface_key,
    apply_focused_vfs_control_key, blur_focused_ink_editor_at,
    blur_focused_vfs_control_for_pointer, commit_focused_ink_clipboard_stream,
    commit_focused_text_editor_stream, push_focused_ink_clipboard_stream,
    push_focused_text_editor_stream, start_focused_ink_clipboard_stream,
    start_focused_text_editor_stream,
};
#[cfg(any(
    all(not(target_arch = "wasm32"), not(test)),
    target_arch = "wasm32",
    test
))]
pub use ink_and_vfs_focus::{close_clipboard_io_step, drive_clipboard_io_step};

use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction;
pub use scene_interaction::{
    close_scene_interaction_step, drive_scene_interaction_step,
    scene_interaction_terminal_is_empty, surface_context_menu_target,
    text_editor_claim_alt_completions, text_editor_claim_menu_action, SurfaceContextMenuTarget,
};
pub(crate) use scene_interaction::{
    progress_presented_input_candidate, PresentedInputCandidateProgress,
};

use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer;
pub use retained_document_consumer::{begin_ui_document_opportunity, UiDocumentFrameCursor};
pub(crate) use retained_document_consumer::{
    render_ui_document_step, set_ui_document_flow, set_ui_presence_notes,
    stamp_ui_document_tree_selected_item,
};

use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading;
pub(crate) use ui_image_loading::current_ui_image_key;
pub use ui_image_loading::{
    apply_ui_image_bytes, framework_widget_context, take_ui_image_asset_fault,
};

use semio_framework_os_renderer_wgpu_ui_engine_dump::introspection;
#[cfg(target_arch = "wasm32")]
pub use introspection::{
    dump_accessibility, dump_board2d, dump_chrome, dump_frame_stats, dump_mesh_stats,
    dump_structure,
};

use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats;

use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use chrome_ledger::native_accessibility_windows;
pub use chrome_ledger::{
    acknowledge_presented_input, begin_accessibility_visible_documents,
    discard_presented_input_candidate, note_accessibility_visible_document,
    note_chrome_accessibility, note_chrome_hit_registry, note_chrome_surfaces,
    note_chrome_walk_parked, note_dispatched_action, note_world3d_live_camera,
    presented_input_candidate_matches, publish_accessibility_visible_documents,
    seal_presented_input_candidate,
};
pub(crate) use chrome_ledger::{
    close_presented_media_slots, drive_presented_media_slots, presented_retained_clock_deadline,
    staged_retained_clock_surfaces, take_world_camera_diagnostic,
    visible_retained_accessibility_target, SHELL_CHROME_ACCESSIBILITY_WINDOW_ID,
};

