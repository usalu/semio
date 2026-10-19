//! 🧩️ Retained UI engine, scene intents and clipboard io.
//!
//! The per-worker retained UI engine cell, the document close queue, the scene intent queue and pointer owners,
//! the retained hit registry, event dispatch, command application, input receipts and the clipboard io pump.

use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::accessibility_window_is_visible;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::{acknowledge_presented_input, begin_accessibility_visible_documents, note_accessibility_visible_document, publish_accessibility_visible_documents, published_accessibility_nodes_for_test, seal_presented_input_candidate};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::complete_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{abort_focused_ink_clipboard_stream, apply_focused_ink_editor_key, apply_focused_vfs_control_key, blur_focused_ink_editor_at, commit_focused_ink_clipboard_stream, commit_focused_text_editor_stream, focus_ink_editor, focus_ink_surface, focused_ink_editor, focused_ink_surface, push_focused_ink_clipboard_stream, push_focused_text_editor_stream, reserve_pending_native_ink_clipboard, rgba_png_data_url, start_focused_ink_clipboard_stream, start_focused_text_editor_stream, with_live_ink_surface, InkClipboardAddress, InkClipboardStream, FOCUSED_INK_EDITOR, FOCUSED_INK_SURFACE, INK_CLIPBOARD_STREAMS, PENDING_NATIVE_INK_CLIPBOARD};
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{close_retiring_focus_clipboard_one, close_window_focus_clipboard_one, focus_vfs_control, FOCUSED_VFS_CONTROL};
#[cfg(all(not(target_arch = "wasm32"), not(any(test, feature = "testkit"))))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{complete_pending_native_text_editor_clipboard, PendingNativeInkClipboardToken, PendingNativeTextEditorClipboardToken};
use semio_framework_os_renderer_wgpu_ui_engine_dump::introspection::{block_list_accessibility_entries, event_feed_accessibility_entries, graph_timeline_accessibility_entries, scene_virtual_accessibility_node_id, table_button_accessibility_entries, table_editable_text_accessibility_entries, table_stepper_accessibility_entries, text_editor_accessibility_entries, vfs_accessibility_entries};
use crate::retained_document_consumer::document_debug_log;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use crate::retained_document_consumer::{apply_retained_document_page, UiDocumentFrameCursor};
use crate::scene_interaction::drive_scene_interaction_step;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use crate::scene_interaction::{close_scene_interaction_step, progress_presented_input_candidate, scene_interaction_terminal_is_empty, PresentedInputCandidateProgress};
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{apply_focused_node_graph_note_key, apply_focused_table_editable_text_key, apply_focused_table_stepper_key, apply_focused_text_editor_key, apply_focused_text_editor_text, blur_focused_table_editable_text_for_pointer, blur_focused_text_editor_for_pointer, focused_table_editable_text_draft, refresh_node_graph_caret_after_pointer, sync_focused_table_editable_text_draft, FocusedTextEditor, FOCUSED_NODE_GRAPH_CARET, FOCUSED_TABLE_EDITABLE_TEXT};
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{blur_text_editor, clear_focused_table_editable_text, commit_focused_table_editable_text_with_engine, focus_table_editable_text, focus_table_stepper, install_text_editor_focus, reset_text_editor_caret, set_focused_table_editable_text_draft, text_editor_focus_in, with_live_text_editor, FOCUSED_TABLE_STEPPER, FOCUSED_TEXT_EDITOR};
use crate::element_contracts::{ScenePointerTarget, WorkerCell, SCENE_POINTER_OWNER_CAPACITY};
use serde_json::Value;
use std::sync::MutexGuard;
use ui_contract::SurfaceId;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use ui_contract::UiDocumentLease;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_events::DragPayload;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_declaration::ui::UiNode;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))] use semio_framework_ui_theme::Theme;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))] use semio_framework_ui_declaration::ui::{UiComponentSceneNode, UiPresence};

//#region RetainedEngineCutover

#[path = "📕️close-scene-pointer-and-dispatch/🦀️.rs"]
mod close_scene_pointer_and_dispatch;
pub use close_scene_pointer_and_dispatch::{
    active_retained_drag_sessions, advance_retained_window_clock, cancel_scene_pointer,
    canvas_catalogue_drag_over, canvas_catalogue_drop, captured_scene_pointer,
    claim_scene_pointer_at, claim_scene_pointer_owner, close_ui_document_one,
    dispatch_presented_ui_pointer_event, dispatch_ui_event, dispatch_ui_pointer_event,
    register_clipped_retained_hit_targets, register_retained_hit_targets, release_scene_pointer,
    request_ui_document_close, retained_any_pointer_capture, retained_content_height,
    retained_pointer_capture_window, retained_scene_target, retained_scene_target_at,
    scene_pointer_target_is_live, tree_window_measures,
};
#[cfg(any(test,feature="testkit"))]
pub use close_scene_pointer_and_dispatch::retained_root_is_tree;
use close_scene_pointer_and_dispatch::{apply_window_clock_caret, stamp_window_clock};

#[path = "♿️accessibility-dispatch-and-focus/🦀️.rs"]
mod accessibility_dispatch_and_focus;
pub use accessibility_dispatch_and_focus::{
    dispatch_accessibility_event, retained_node_focus_identity, retained_node_has_visible_focus,
    retained_node_is_select, RetainedNodeFocusKind,
};

#[path = "📋️receipts-drop-and-clipboard/🦀️.rs"]
mod receipts_drop_and_clipboard;
pub use receipts_drop_and_clipboard::{
    apply_drop_committed, apply_scene_ui_command, apply_ui_commands, decode_drop_payload,
    drop_target_action, pointer_button_code, reserve_retained_input_receipt,
    settle_retained_input_receipt, RETAINED_INPUT_RECEIPTS,
};
#[cfg(any(test,feature="testkit"))]
pub use receipts_drop_and_clipboard::{
    merge_action_args, pointer_button_from_code, RetainedInputReceiptReservation,
};
#[cfg(all(not(target_arch="wasm32"),any(test,feature="testkit")))]
pub use receipts_drop_and_clipboard::apply_clipboard_paste_requested;
#[cfg(target_arch="wasm32")]
pub use receipts_drop_and_clipboard::apply_clipboard_paste_requested;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🛍️wgpu-app-catalogue/🦀️.rs"]
mod app_catalogue_tests;
//#endregion RetainedEngineCutover

pub use semio_framework_os_renderer_wgpu_contracts::{
    component_scene_is_presented, install_ui_engine_locale, ui_document_close_pending, ui_document_close_pending_for, ScenePointerOwner, ScenePointerOwners, UiDocumentCloseOwner, UiDocumentCloseQueue, UiEngineCell, UiEngineGuard, SCENE_POINTER_OWNERS, UI_DOCUMENT_CLOSE_QUEUE, UI_ENGINE, UI_ENGINE_LOCALE_UNRESOLVED,
};
#[cfg(any(test, feature = "testkit"))]
pub use semio_framework_os_renderer_wgpu_contracts::TEST_UI_ENGINE_LOCALE;

pub use semio_framework_os_renderer_wgpu_ui_engine_focus::scene_intents_and_clipboard::*;
