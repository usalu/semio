//! 📄️ DocumentState component of the shell: sessions, window documents and retirement.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as protocol;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
use semio_framework_os_renderer_wgpu_shell_async::debug_log;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogRequest, ChromePrefsState, ChromeState, ThemeDocument, appearance_id, default_compute_worker_count, format_keybinding_shortcut};
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::ShellDirectoryHomePublicationOutcome;
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, SHELL_CHROME_IO_FIELD_BYTES};
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, DockTabNode, PrefsState, ResolvedCommand, ShellConflictRow, TimeTravelVerb, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, capture_ui_prefs_snapshot, command_address_stable_key, custom_themes_from, display_panel_body, env_lock, read_ui_preferences, resolve_appearance_id, resolve_locale_id, resolve_ui_driver_chrome, settings_tree_select_item_with, stored_field_get, theme_from_document, time_travel_band_controls, with_chrome_prefs};
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, TutorialState};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellPoolFuture;
use semio_framework_os_renderer_wgpu_shell_sync::SyncState;
use semio_framework::{AppDefinition, IconName, ViewSessionIdentity};
use semio_framework::kernel::UiDirtyScope;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
use semio_framework_os_config::opening_config::UiChromeLayout as OsUiChromeLayout;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::{CanonicalDirectoryEventPageV1, DirectoryEventPageAckV1};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_ui_viewport::{Viewport3dAxonometricHemisphere, Viewport3dAxonometricQuadrant, Viewport3dAxonometricVariant, Viewport3dCurvilinearMapping, Viewport3dObliqueVariant, Viewport3dOrthographicView, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use semio_framework_value::DslValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use ui_contract::{SurfaceId, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS, UiDocumentLease};
use ui_wgpu::wgpu::{ActionDescriptor, PointerModifiers, Theme, UiButtonNode, UiControlNode, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧬️component/🦀️.rs"]
mod staged_docs__component;
pub use staged_docs__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_docs__keyboard_and_palette;
pub use staged_docs__keyboard_and_palette::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/⏪️time-travel/🦀️.rs"]
mod staged_docs__time_travel;
pub use staged_docs__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_docs__settings_and_dock_persistence;
pub use staged_docs__settings_and_dock_persistence::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_docs__theme_file_import_and_accessibility;
pub use staged_docs__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_docs__space_admin_and_settle;
pub use staged_docs__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_docs__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/📎️local-folders/🦀️.rs"]
mod staged_docs__local_folders;
pub use staged_docs__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/📨️command-dispatch/🦀️.rs"]
mod staged_docs__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🔖️identity-and-types/🦀️.rs"]
mod staged_docs__identity_and_types;
pub use staged_docs__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🖱️pointer-input/🦀️.rs"]
mod staged_docs__pointer_input;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_docs__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_docs__chrome_overlays_and_tour;
pub use staged_docs__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧩️session-and-topology/🦀️.rs"]
mod staged_docs__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_docs__window_actions_and_commands;
pub use staged_docs__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_docs__chrome_paint_and_navbar;
pub use staged_docs__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧵️shell-async-runners/🦀️.rs"]
mod staged_docs__shell_async_runners;
pub use staged_docs__shell_async_runners::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🧾️panel-records/🦀️.rs"]
mod staged_docs__panel_records;
pub use staged_docs__panel_records::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📄️docs/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_docs__storage_prefs_and_i18n;
pub use staged_docs__storage_prefs_and_i18n::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
