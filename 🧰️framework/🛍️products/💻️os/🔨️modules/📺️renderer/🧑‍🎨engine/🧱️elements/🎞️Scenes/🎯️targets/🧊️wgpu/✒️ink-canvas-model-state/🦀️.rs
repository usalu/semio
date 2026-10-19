//! ✒️ Ink canvas model and interaction state.

use super::*;

//#region InkCanvas
// 📝️ Direct DrawList painting for ink-canvas, ported from ink-canvas-host.tsx (framework/renderer/react).

#[path = "🧾️ink-document-model-and-edit-helpers/🦀️.rs"]
mod ink_document_model_and_edit_helpers;
use ink_document_model_and_edit_helpers::{
    activate_ink_edit, create_ink_host_id, ink_double_click_edit, ink_item_with_position,
};
pub use ink_document_model_and_edit_helpers::{
    create_ink_item, erase_ink_stroke_points_in_item, ink_hits_point, ink_item_id, ink_item_kind,
    ink_item_locked, ink_item_num, ink_maybe_snap_fields, ink_resize_bounds, ink_screen_to_world,
    scale_ink_item, InkCameraF, InkCameraJson, InkDocumentJson,
};
pub(crate) use ink_document_model_and_edit_helpers::{
    ink_edit_control_id, ink_edit_for_point, ink_edit_screen_rect, ink_item_bounds, ink_item_str,
    ink_points, ink_snap_coordinate,
};

#[path = "🔎️ink-interaction-document-and-block-scanning/🦀️.rs"]
mod ink_interaction_document_and_block_scanning;
pub use ink_interaction_document_and_block_scanning::{
    collect_ink_document_block_spans, InkInteractionDocument, INK_EVENT_JSON_BYTE_CAPACITY,
    INK_INTERACTION_DOCUMENT_BYTE_CAPACITY, INK_INTERACTION_ITEM_CAPACITY,
};
use ink_interaction_document_and_block_scanning::INK_SELECTION_ITEM_CAPACITY;

#[path = "📤️ink-event-pages-and-edit-routing/🦀️.rs"]
mod ink_event_pages_and_edit_routing;
pub use ink_event_pages_and_edit_routing::{
    cancel_ink_edit, ink_edit_active, ink_edit_blur_into, ink_edit_key_into, ink_gesture_abort_into,
    InkEventJsonPages, InkRawPages,
};
use ink_event_pages_and_edit_routing::{
    ink_interaction_domain, ink_interaction_id, write_ink_events_action,
    write_ink_interaction_hover, write_ink_interaction_selection, InkOwnedRaw,
};
pub(crate) use ink_event_pages_and_edit_routing::clear_ink_pointer_state;

#[path = "📋️ink-clipboard-camera-and-wheel/🦀️.rs"]
mod ink_clipboard_camera_and_wheel;
pub use ink_clipboard_camera_and_wheel::{
    ink_clipboard_copy_text, ink_clipboard_paste_into, ink_clipboard_target, ink_edit_paste_into,
    ink_pointer_cancel_into, ink_wheel_into, validate_ink_value, InkClipboardContentKind,
};
#[cfg(any(test,feature="testkit"))]
pub use ink_clipboard_camera_and_wheel::ink_pointer_state_is_clear;
use ink_clipboard_camera_and_wheel::{checked_ink_document, write_ink_camera_action};

#[path = "🔁️ink-interaction-job-scan/🦀️.rs"]
mod ink_interaction_job_scan;
pub use ink_interaction_job_scan::{
    InkBlockCursor, InkInteractionEvent, InkInteractionJob, InkInteractionStep,
};

#[path = "🏁️ink-interaction-job-publish-and-close/🦀️.rs"]
mod ink_interaction_job_publish_and_close;
pub use ink_interaction_job_publish_and_close::INK_RESIZE_HANDLES;

//#endregion InkCanvas
