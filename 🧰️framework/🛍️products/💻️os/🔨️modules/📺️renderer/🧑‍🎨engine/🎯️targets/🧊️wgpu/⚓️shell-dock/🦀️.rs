//! ⚓️ ShellDockState component of the shell: dock model, tabs, panel anchors and layouts.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, instant_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, DOCK_SKELETON_VERSION, DockSkeleton, DockUiPanelState, DockUiState};
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, FRAMEWORK_SYNC_PANEL_TAB_ID, WORLD_PROJECTION_TEMPLATES};
use semio_framework_os_renderer_wgpu_shell_history::{OS_SHELL_CONFIG_MAX_BYTES, OS_SHELL_CONFIG_STORAGE_KEY, empty_os_shell_config, raw_prefs_get, raw_prefs_set};
use semio_framework_os_renderer_wgpu_shell_plugins::{PluginState, flatten_panel_tab_leaves};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, ChromeGroupItem, DockTabNode, PrefsState, commit_ui_preferences_write, display_panel_body, labels, read_ui_preferences, replay_ui_preferences};
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState};
use semio_framework_os_renderer_wgpu_shell_sync::{ShellSyncPill, ShellSyncRemote, SyncState};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WINDOW_PANE_CHIP_PARENT, WORLD_PROJECTION_PANE_PARENT, WindowIconOverride, WindowPaneChip, WindowState, WindowTitleOverride, world_projection_template, world_projection_template_selection_id};
use semio_framework::{AppDefinition, IconName};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_os_config::opening_config::{UiPreferences, UserNamedLayout};
use semio_framework_os_config::opening_config::mutations::{UiPreferencesConfigMutation, set_named_layout};
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockState, WindowSilhouette};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use semio_framework_value::{FromValue, ToValue};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use semio_framework_ui_declaration::layout::{ActionDescriptor, WindowLayout, WindowStackCorner};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, InputState};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::{UiButtonNode, UiControlNode, UiInputNode, UiNode, UiPresence, UiTreeItemNode, UiTreeSectionNode, UiTreeWindowRowExtent};
use semio_framework_ui_declaration::utilities::UtilityNode;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🧬️component/🦀️.rs"]
mod staged_dock__component;
pub use staged_dock__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_dock__keyboard_and_palette;
pub use staged_dock__keyboard_and_palette::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_dock__dock_model_and_measures;
pub use staged_dock__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_dock__settings_and_dock_persistence;
pub use staged_dock__settings_and_dock_persistence::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🎓️tutorial-runtime/🦀️.rs"]
mod staged_dock__tutorial_runtime;
pub use staged_dock__tutorial_runtime::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🎬️shell-actions/🦀️.rs"]
mod staged_dock__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_dock__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_dock__storage_prefs_and_i18n;
pub use staged_dock__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🔖️identity-and-types/🦀️.rs"]
mod staged_dock__identity_and_types;
pub use staged_dock__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🖱️pointer-input/🦀️.rs"]
mod staged_dock__pointer_input;
pub use staged_dock__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_dock__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_dock__chrome_overlays_and_tour;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🐚️shell-parent/🦀️.rs"]
mod staged_dock__shell_parent;
pub use staged_dock__shell_parent::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🧩️session-and-topology/🦀️.rs"]
mod staged_dock__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_dock__window_actions_and_commands;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_dock__chrome_paint_and_navbar;
pub use staged_dock__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🧾️panel-records/🦀️.rs"]
mod staged_dock__panel_records;
pub use staged_dock__panel_records::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/⚓️dock/🪟️tree-windows/🦀️.rs"]
mod staged_dock__tree_windows;
pub use staged_dock__tree_windows::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
