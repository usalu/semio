//! 🎚️ ChromeState component of the shell: notices, dialogs, tour, tooltips and agent approvals.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_framework_os_config::opening_config::{UiAppearance as OsUiAppearance, UiDriver as OsUiDriver};
use semio_framework_os_renderer_wgpu_contracts::ShellFindItems;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::RetainedNodeFocusKind;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::{DslValue, FromValue, ToValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use semio_framework_ui_declaration::layout::{ActionDescriptor, FRAMEWORK_PANEL_TAB_HISTORY_ID};
use semio_framework_ui_wgpu_input::InputState;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🧬️component/🦀️.rs"]
mod staged_chrome_state__component;
pub use staged_chrome_state__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_chrome_state__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/⏪️time-travel/🦀️.rs"]
mod staged_chrome_state__time_travel;
pub use staged_chrome_state__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_chrome_state__dock_model_and_measures;
pub use staged_chrome_state__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_chrome_state__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_chrome_state__theme_file_import_and_accessibility;
pub use staged_chrome_state__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_chrome_state__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_chrome_state__storage_prefs_and_i18n;
pub use staged_chrome_state__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🔖️identity-and-types/🦀️.rs"]
mod staged_chrome_state__identity_and_types;
pub use staged_chrome_state__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🖱️pointer-input/🦀️.rs"]
mod staged_chrome_state__pointer_input;
pub use staged_chrome_state__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_chrome_state__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_chrome_state__chrome_overlays_and_tour;
pub use staged_chrome_state__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_chrome_state__window_actions_and_commands;
pub use staged_chrome_state__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_chrome_state__chrome_paint_and_navbar;
pub use staged_chrome_state__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🎚️chrome-state/🧾️panel-records/🦀️.rs"]
mod staged_chrome_state__panel_records;
pub use staged_chrome_state__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
