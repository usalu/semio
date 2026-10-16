//! 🎛️ PrefsState component of the shell: preferences, themes, drivers and storage door.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_renderer_wgpu_shell_async::{debug_log, resolve_language_axes};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromePrefsState, ChromeState, RefusedGuestFault, ThemeDocument, ThemeNumber, ThemePaintRef, UiPrefsSnapshot, appearance_id, apply_theme_document_metrics, default_compute_worker_count, shell_theme_document_base, theme_document_geometry};
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, SHELL_CHROME_IO_FIELD_BYTES, prefs_get, prefs_set, raw_prefs_get, raw_prefs_set};
use semio_framework_os_renderer_wgpu_shell_plugins::{DEFAULT_APP_NONE_VALUE, PluginState, ShellPluginFault, ShellSurfaceFault, flatten_panel_tab_leaves};
use semio_framework::{AppDefinition, IconName, ViewModel};
use semio_framework_os_config::io::text::mutations::decode_ui_preferences_config_mutation_json;
use semio_framework_os_config::opening_config::{UI_PREFERENCES_CONFIG_SCHEMA, UiAppearance as OsUiAppearance, UiChromeLayout as OsUiChromeLayout, UiDriver as OsUiDriver, UiLocale as OsUiLocale, UiPreferences, UiTheme as OsUiTheme, apply_ui_preferences_config_mutation};
use semio_framework_os_config::opening_config::mutations::{UiPreferencesConfigMutation, set_appearance, set_custom_driver, set_custom_theme, set_driver, set_keybinding_override, set_layout, set_locale, set_terminology, set_theme};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_value::{DslValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use ui_wgpu::wgpu::{ActionDescriptor, FontAtlas, HitKind, Rect, Rgba, Theme, UiButtonNode, UiControlNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSectionNode, UiSelectItem, UiSelectNode, UiStackNode, UiTextNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧬️component/🦀️.rs"]
mod staged_prefs__component;
pub use staged_prefs__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/⏪️time-travel/🦀️.rs"]
mod staged_prefs__time_travel;
pub use staged_prefs__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_prefs__dock_model_and_measures;
pub use staged_prefs__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_prefs__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_prefs__theme_file_import_and_accessibility;
pub use staged_prefs__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/📎️local-folders/🦀️.rs"]
mod staged_prefs__local_folders;
pub use staged_prefs__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/📨️command-dispatch/🦀️.rs"]
mod staged_prefs__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_prefs__storage_prefs_and_i18n;
pub use staged_prefs__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🔖️identity-and-types/🦀️.rs"]
mod staged_prefs__identity_and_types;
pub use staged_prefs__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_prefs__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_prefs__chrome_overlays_and_tour;
pub use staged_prefs__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧩️session-and-topology/🦀️.rs"]
mod staged_prefs__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_prefs__window_actions_and_commands;
pub use staged_prefs__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_prefs__chrome_paint_and_navbar;
pub use staged_prefs__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎛️prefs/🧾️panel-records/🦀️.rs"]
mod staged_prefs__panel_records;
pub use staged_prefs__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
