//! ✍️ Text editor scene document, delivery and interaction.
//!
//! The editor scene document, the typing delivery state and echo ledger, the explicit draft settings and the public
//! text editor pointer, key, wheel and action API.

use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{ENGINE_SURFACES, ENGINE_SURFACE_CAPACITY, EngineSurfaceRegistry, EngineSurfaceToken, TEXT_EDITOR_PENDING_ECHO_CAPACITY, TextEditorDeliveryInFlight, TextEditorDeliveryPublication, TextEditorDeliverySnapshot, TextEditorDraftPublication, TextEditorTypingEnd, TextEditorTypingRun, engine_canvas_fault_log, retire_string_page, text_editor_now_ms};
use framework_editor::EditorHost;
#[cfg(target_arch = "wasm32")]
use js_sys;
use serde_json::{json, Value};
use std::collections::VecDeque;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::{KeyAction, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_declaration::ui::UiComponentSceneNode;

#[path = "📄️scene-document-and-host-sync/🦀️.rs"]
mod scene_document_and_host_sync;
pub use scene_document_and_host_sync::{
    synchronize_text_editor_host, text_editor_scene_document, TEXT_EDITOR_SCENE_OWNED_BYTES,
};

#[path = "📤️draft-actions-and-outbox/🦀️.rs"]
mod draft_actions_and_outbox;
pub(crate) use draft_actions_and_outbox::TextEditorExplicitDraftSettings;
pub use draft_actions_and_outbox::{
    drive_text_editor_outbox_step, end_every_text_editor_typing, end_text_editor_typing,
    has_pending_text_editor_outbox, text_editor_explicit_draft_settings, text_editor_wheel_into,
    TextEditorActionOutcome,
};
use draft_actions_and_outbox::{emit_text_editor_actions, text_editor_receipt_token};
#[cfg(target_arch="wasm32")]
pub use draft_actions_and_outbox::semio_wgpu_host_page_hidden;

#[path = "🧾️receipts-and-explicit-draft/🦀️.rs"]
mod receipts_and_explicit_draft;
pub use receipts_and_explicit_draft::{
    drive_text_editor_retained_action_step, settle_text_editor_action_receipt,
    text_editor_apply_explicit_draft_into, text_editor_cancel_explicit_publication,
    text_editor_discard_explicit_draft, text_editor_explicit_draft_status,
    text_editor_move_explicit_draft_history, TextEditorRetainedActionStep,
};
use receipts_and_explicit_draft::{text_editor_action_bytes, write_text_editor_selection};
pub(crate) use receipts_and_explicit_draft::{
    TextEditorExplicitDraftStatus, TextEditorRetainedActionBatch,
};

#[path = "🖋️editing-and-pointer-api/🦀️.rs"]
mod editing_and_pointer_api;
pub use editing_and_pointer_api::{
    text_editor_accessibility_value, text_editor_apply_completion_into, text_editor_apply_key_into,
    text_editor_caret, text_editor_caret_screen, text_editor_explicit_draft_has_conflict,
    text_editor_explicit_draft_is_dirty, text_editor_is_read_only, text_editor_pointer_button_into,
    text_editor_pointer_cancel_into, text_editor_pointer_click_into, text_editor_pointer_move_into,
    text_editor_preview_rename, text_editor_replace_all_into, text_editor_replace_selection_into,
    text_editor_select_span_into, text_editor_selection_text, text_editor_set_selection_into,
};
