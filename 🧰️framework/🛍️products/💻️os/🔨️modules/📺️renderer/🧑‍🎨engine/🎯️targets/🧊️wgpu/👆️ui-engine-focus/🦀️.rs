//! 👆️ Focus and clipboard of the UI engine: text editor, node graph caret and table focus, virtual file system, ink editor and ink clipboard focus, and the scene intent queue with the mounted clipboard io.
//!
//! 🧭️ Native and wasm32: the focus state of the focusable scene surfaces, the caret application, the scene intent queue and the native clipboard streams that follow a focus change. The module files keep their place in the interpreter tree and are mounted here as public modules.

extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
use crate::element_contracts::WorkerCell;
#[cfg(not(target_arch = "wasm32"))]
use crate::ink_and_vfs_focus::complete_pending_native_ink_clipboard;
#[cfg(all(not(target_arch = "wasm32"), not(test)))]
use crate::ink_and_vfs_focus::{complete_pending_native_text_editor_clipboard, PendingNativeInkClipboardToken, PendingNativeTextEditorClipboardToken};
use semio_framework_os_renderer_wgpu_contracts::{ScenePointerOwner, UI_ENGINE};
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_declaration::ui::UiNode;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/✍️text-and-table-focus/🦀️.rs"]
pub mod text_and_table_focus;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖋️ink-and-vfs-focus/🦀️.rs"]
pub mod ink_and_vfs_focus;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🎬️scene-intents-and-clipboard/🦀️.rs"]
pub mod scene_intents_and_clipboard;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
