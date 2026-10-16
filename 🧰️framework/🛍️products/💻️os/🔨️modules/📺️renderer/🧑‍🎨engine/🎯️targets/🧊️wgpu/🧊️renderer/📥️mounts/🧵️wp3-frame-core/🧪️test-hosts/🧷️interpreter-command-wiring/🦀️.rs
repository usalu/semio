//! 🧷 Test host of the laws in `🔬️wgpu-ui-command-wiring`: they name the monolith (scenes, engine canvas, the frame fixtures), so they run in the renderer test unit with the namespace of the `retained_ui_engine` module of the `ui-engine` partition around them.

#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::apply_retained_document_page;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::UiDocumentFrameCursor;
pub(crate) use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::*;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::close_scene_interaction_step;
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::drive_scene_interaction_step;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::progress_presented_input_candidate;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::scene_interaction_terminal_is_empty;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine::scene_interaction::PresentedInputCandidateProgress;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::acknowledge_presented_input;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_accessibility_visible_document;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_nodes_for_test;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::seal_presented_input_candidate;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::abort_focused_ink_clipboard_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_ink_editor_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::apply_focused_vfs_control_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::blur_focused_ink_editor_at;
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::close_window_focus_clipboard_one;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::commit_focused_ink_clipboard_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::commit_focused_text_editor_stream;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::complete_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::focus_ink_editor;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::focus_ink_surface;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::focused_ink_editor;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::focused_ink_surface;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::push_focused_ink_clipboard_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::push_focused_text_editor_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::reserve_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::rgba_png_data_url;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::start_focused_ink_clipboard_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::start_focused_text_editor_stream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::with_live_ink_surface;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::InkClipboardAddress;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::InkClipboardStream;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::FOCUSED_INK_EDITOR;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::FOCUSED_INK_SURFACE;
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::FOCUSED_VFS_CONTROL;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::INK_CLIPBOARD_STREAMS;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::PENDING_NATIVE_INK_CLIPBOARD;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_node_graph_note_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_table_editable_text_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_table_stepper_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_key;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::apply_focused_text_editor_text;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_table_editable_text_for_pointer;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::blur_focused_text_editor_for_pointer;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::clear_focused_table_editable_text;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::focus_table_editable_text;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::focus_table_stepper;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::focused_table_editable_text_draft;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::install_text_editor_focus;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::refresh_node_graph_caret_after_pointer;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::set_focused_table_editable_text_draft;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::sync_focused_table_editable_text_draft;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::text_editor_focus_in;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::FocusedTextEditor;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::FOCUSED_NODE_GRAPH_CARET;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::FOCUSED_TABLE_EDITABLE_TEXT;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::FOCUSED_TABLE_STEPPER;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::FOCUSED_TEXT_EDITOR;
use semio_framework_ui_arena::NodeId;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_ui_declaration::ui::UiComponentSceneNode;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_ui_layout::layout::ActionDescriptor;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_ui_layout::layout::UiPresence;
use semio_framework_ui_theme::geometry::Rect;
#[cfg(all(not(target_arch = "wasm32"), test))]
use semio_framework_ui_theme::Theme;
use serde_json::Value;
use ui_contract::SurfaceId;
#[cfg(all(not(target_arch = "wasm32"), test))]
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::DragPayload;

#[path = "../../../../../../../🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"]
mod tests;
