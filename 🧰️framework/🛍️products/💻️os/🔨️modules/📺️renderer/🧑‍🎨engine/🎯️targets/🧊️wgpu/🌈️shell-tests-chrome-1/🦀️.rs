//! 🌈️ Companion test bin of the wgpu renderer: chrome and views (1).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_async::CancelToken;
use semio_framework_os_renderer_wgpu_contracts::{SCENE_POINTER_OWNERS, ScenePointerOwners, UI_ENGINE, UiDocumentCloseOwner, UiDocumentCloseQueue, ui_document_close_pending};
use semio_framework_os_renderer_wgpu_engine_attach::{engine_surface_attach, retire_engine_surface_fixture};
use semio_framework_os_renderer_wgpu_engine_graph::node_graph;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard;
use semio_framework_os_renderer_wgpu_engine_text::text_editor;
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, collect_fixture_actions, settle_renderer_action_receipt};
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_scene_feeds::{
    acknowledge_block_list_accessibility_candidates, acknowledge_event_feed_accessibility_candidates, acknowledge_graph_timeline_accessibility_candidates, acknowledge_table_button_accessibility_candidates,
    acknowledge_table_editable_text_accessibility_candidates, acknowledge_table_stepper_accessibility_candidates, acknowledge_vfs_accessibility_candidates, acknowledge_world3d_accessibility_candidates, graph_timeline_accessibility_controls,
    graph_timeline_scene_label_track, seal_block_list_accessibility_candidates, seal_event_feed_accessibility_candidates, seal_graph_timeline_accessibility_candidates, seal_table_button_accessibility_candidates,
    seal_table_editable_text_accessibility_candidates, seal_table_stepper_accessibility_candidates, seal_vfs_accessibility_candidates, seal_world3d_accessibility_candidates, stage_block_list_accessibility_controls,
    stage_event_feed_accessibility_controls, stage_graph_timeline_accessibility_controls, stage_table_button_accessibility_cells, stage_table_editable_text_accessibility_cells, stage_table_stepper_accessibility_cells,
    stage_vfs_accessibility_controls, stage_world3d_accessibility_controls,
};
use semio_framework_os_renderer_wgpu_scene_ink::{InkInteractionEvent, InkInteractionJob, ink_pointer_state_is_clear};
use semio_framework_os_renderer_wgpu_scene_render::{cancel_canvas_pointer_gesture, cancel_canvas_pointer_gesture_for, request_canvas_pointer_gesture_cancel_for_window, scene_has_bespoke_pointer_dispatch};
use semio_framework_os_renderer_wgpu_scene_state::{
    BlockListAccessibilityControl, EventFeedAccessibilityControl, TableEditableTextFocusTarget, TableStepperFocusTarget, VfsAccessibilityControl, VfsAccessibilityControlKind, World3dAccessibilityControl, mount_scene_identity, remember_scene_theme,
};
use semio_framework_os_renderer_wgpu_scene_tables::{merge_action_args, table_button_accessibility_cells, table_editable_text_accessibility_cells, table_stepper_accessibility_cells};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::UiDocumentFrameCursor;
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::apply_retained_document_page;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::*;
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::drive_scene_interaction_step;
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::{PresentedInputCandidateProgress, close_scene_interaction_step, progress_presented_input_candidate, scene_interaction_terminal_is_empty};
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::{
    acknowledge_presented_input, begin_accessibility_visible_documents, note_accessibility_visible_document, publish_accessibility_visible_documents, published_accessibility_nodes_for_test, seal_presented_input_candidate,
};
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{
    FOCUSED_INK_EDITOR, FOCUSED_INK_SURFACE, INK_CLIPBOARD_STREAMS, InkClipboardAddress, InkClipboardStream, PENDING_NATIVE_INK_CLIPBOARD, abort_focused_ink_clipboard_stream, apply_focused_ink_editor_key, apply_focused_vfs_control_key,
    blur_focused_ink_editor_at, commit_focused_ink_clipboard_stream, commit_focused_text_editor_stream, complete_pending_native_ink_clipboard, focus_ink_editor, focus_ink_surface, focused_ink_editor, focused_ink_surface,
    push_focused_ink_clipboard_stream, push_focused_text_editor_stream, reserve_pending_native_ink_clipboard, rgba_png_data_url, start_focused_ink_clipboard_stream, start_focused_text_editor_stream, with_live_ink_surface,
};
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::{FOCUSED_VFS_CONTROL, close_window_focus_clipboard_one};
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{
    FOCUSED_NODE_GRAPH_CARET, FOCUSED_TABLE_EDITABLE_TEXT, FocusedTextEditor, apply_focused_node_graph_note_key, apply_focused_table_editable_text_key, apply_focused_table_stepper_key, apply_focused_text_editor_key, apply_focused_text_editor_text,
    blur_focused_table_editable_text_for_pointer, blur_focused_text_editor_for_pointer, focused_table_editable_text_draft, refresh_node_graph_caret_after_pointer, sync_focused_table_editable_text_draft,
};
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{
    FOCUSED_TABLE_STEPPER, FOCUSED_TEXT_EDITOR, clear_focused_table_editable_text, focus_table_editable_text, focus_table_stepper, install_text_editor_focus, set_focused_table_editable_text_draft, text_editor_focus_in,
};
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_arena::NodeId;
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_ui_layout::layout::ActionDescriptor;
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_ui_layout::layout::UiPresence;
use semio_framework_ui_locale::{Locale, Terminology};
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_ui_theme::Theme;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_value::DslValue;
use serde_json::Value;
use std::future::Future;
use ui_contract::{SurfaceId, UiDocumentLease, UiFixedList, UiText};
use ui_wgpu::wgpu::{DragPayload, FontAtlas, InputState, KeyAction, PointerModifiers, UiStackNode};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"]
mod ui_command_wiring_tests;
