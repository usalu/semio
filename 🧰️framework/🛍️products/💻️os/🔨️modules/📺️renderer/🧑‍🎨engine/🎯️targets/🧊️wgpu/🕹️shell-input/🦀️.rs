//! 🕹️ ShellInputState component of the shell: pointer, selection, context menu and retained hits.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogFieldKind, ChromeDialogRequest, ChromeDialogStop, ChromeState, ThemeDocument, appearance_id, format_keybinding_shortcut, shell_shortcut_table, theme_geometry};
use semio_framework_os_renderer_wgpu_shell_dock::{ShellDockState, retained_event_modifiers};
use semio_framework_os_renderer_wgpu_shell_docs::{COMPOSE_WINDOW_TEMPLATE_MIME, DocumentState, WindowTemplateDragPayload, custom_theme_document_from};
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, TimeTravelFocus};
use semio_framework_os_renderer_wgpu_shell_prefs::{PrefsState, dsl_value_as_json, settings_section, settings_text_row, theme_paint_editor_values};
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WindowState};
use semio_framework::IconName;
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragPayload, WindowSilhouette};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use semio_framework_value::{DslValue, FromValue};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use ui_wgpu::wgpu::{ActionDescriptor, HitKind, HitTarget, InputState, PointerModifiers, Rect, Rgba, Theme, TreeDragState, UiButtonNode, UiControlNode, UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiStackNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent, WidgetInteractionMaps, WindowStackCorner};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧬️component/🦀️.rs"]
mod staged_input__component;
pub use staged_input__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_input__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/⏪️time-travel/🦀️.rs"]
mod staged_input__time_travel;
pub use staged_input__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_input__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_input__theme_file_import_and_accessibility;
pub use staged_input__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_input__space_admin_and_settle;
pub use staged_input__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_input__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_input__storage_prefs_and_i18n;
pub use staged_input__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🔖️identity-and-types/🦀️.rs"]
mod staged_input__identity_and_types;
pub use staged_input__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🖱️pointer-input/🦀️.rs"]
mod staged_input__pointer_input;
pub use staged_input__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_input__chrome_frame_and_main_window;
pub use staged_input__chrome_frame_and_main_window::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_input__chrome_overlays_and_tour;
pub use staged_input__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧩️session-and-topology/🦀️.rs"]
mod staged_input__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_input__window_actions_and_commands;
pub use staged_input__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_input__chrome_paint_and_navbar;
pub use staged_input__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🕹️input/🧾️panel-records/🦀️.rs"]
mod staged_input__panel_records;
pub use staged_input__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
