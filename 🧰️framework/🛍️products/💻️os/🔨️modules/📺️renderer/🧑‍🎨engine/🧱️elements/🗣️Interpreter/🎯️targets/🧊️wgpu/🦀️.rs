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

#[cfg(test)]
use crate::element_contracts::WorkerCell;
#[cfg(test)]
use ui_wgpu::wgpu::UiNode;

#[path = "🧮️render-plan-validator/🦀️.rs"]
mod render_plan_validator;
pub use render_plan_validator::{validate_component_scene, RenderPlanLimits, RENDER_PLAN_LIMITS};
#[cfg(test)]
pub use render_plan_validator::{validate_ui_node, validate_window_body_surface};

#[path = "🧩️retained-ui-engine/🦀️.rs"]
mod retained_ui_engine;
#[cfg(test)]
pub(crate) use retained_ui_engine::retained_root_is_tree;
#[cfg(test)]
use retained_ui_engine::UI_ENGINE;
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

#[path = "✍️text-and-table-focus/🦀️.rs"]
mod text_and_table_focus;
pub(crate) use text_and_table_focus::sync_focused_table_editable_text_draft;
pub use text_and_table_focus::{
    apply_focused_node_graph_note_key, apply_focused_table_editable_text_key,
    apply_focused_table_stepper_key, apply_focused_text_editor_key, apply_focused_text_editor_text,
    blur_focused_node_graph_caret_for_pointer, blur_focused_table_editable_text_for_pointer,
    blur_focused_table_stepper_for_pointer, blur_focused_text_editor_for_pointer,
    refresh_node_graph_caret_after_pointer,
};

#[path = "🖋️ink-and-vfs-focus/🦀️.rs"]
mod ink_and_vfs_focus;
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

#[path = "🖱️scene-interaction/🦀️.rs"]
mod scene_interaction;
pub use scene_interaction::{
    close_scene_interaction_step, drive_scene_interaction_step,
    scene_interaction_terminal_is_empty, surface_context_menu_target,
    text_editor_claim_alt_completions, text_editor_claim_menu_action, SurfaceContextMenuTarget,
};
pub(crate) use scene_interaction::{
    progress_presented_input_candidate, PresentedInputCandidateProgress,
};

#[path = "📄️retained-document-consumer/🦀️.rs"]
mod retained_document_consumer;
pub use retained_document_consumer::{begin_ui_document_opportunity, UiDocumentFrameCursor};
pub(crate) use retained_document_consumer::{
    render_ui_document_step, set_ui_document_flow, set_ui_presence_notes,
    stamp_ui_document_tree_selected_item,
};

#[path = "🖼️ui-image-loading/🦀️.rs"]
mod ui_image_loading;
pub(crate) use ui_image_loading::current_ui_image_key;
#[cfg(test)]
pub(crate) use ui_image_loading::resolve_ui_image;
pub use ui_image_loading::{
    apply_ui_image_bytes, framework_widget_context, take_ui_image_asset_fault,
};

#[path = "🔬️introspection/🦀️.rs"]
mod introspection;
#[cfg(target_arch = "wasm32")]
pub use introspection::{
    dump_accessibility, dump_board2d, dump_chrome, dump_frame_stats, dump_mesh_stats,
    dump_structure,
};

#[path = "🧱️mesh-and-board2d-stats/🦀️.rs"]
mod mesh_and_board2d_stats;

#[path = "🎯️chrome-ledger/🦀️.rs"]
mod chrome_ledger;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use chrome_ledger::native_accessibility_windows;
#[cfg(test)]
pub(crate) use chrome_ledger::{
    accessibility_visible_window_ids, candidate_tree_item_selected_for_test,
    published_accessibility_nodes_for_test, published_accessibility_target_for_test,
};
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

#[cfg(test)]
/// 🔎️ The whole Interpreter module as the one text the unsplit file held, minus the shared contracts: this file and every child, for the source-text laws.
pub(crate) const INTERPRETER_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🧮️render-plan-validator/🦀️.rs"),
    include_str!("🧩️retained-ui-engine/🦀️.rs"),
    include_str!("✍️text-and-table-focus/🦀️.rs"),
    include_str!("🖋️ink-and-vfs-focus/🦀️.rs"),
    include_str!("🖱️scene-interaction/🦀️.rs"),
    include_str!("📄️retained-document-consumer/🦀️.rs"),
    include_str!("🖼️ui-image-loading/🦀️.rs"),
    include_str!("🔬️introspection/🦀️.rs"),
    include_str!("🧱️mesh-and-board2d-stats/🦀️.rs"),
    include_str!("🎯️chrome-ledger/🦀️.rs"),
);

#[cfg(test)]
#[path = "../../🧵️worker-cell/🧪️tests/🦀️.rs"]
mod worker_cell_tests;
