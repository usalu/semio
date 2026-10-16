//! 🚶️ Shell use-cases: session, topology, persistence, time travel and tutorial runtime.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, DOCK_SKELETON_VERSION, DockSkeleton, DockTabSkeleton, DockUiState, reference_selection_ids, window_pane_owner};
use semio_framework_os_renderer_wgpu_shell_commands::{CommandState, FUZZY_SEARCH_LIMIT, FUZZY_SEARCH_THRESHOLD, rank_fuzzy_items};
use semio_framework_os_renderer_wgpu_shell_directory::{DirectoryState, HISTORY_MUTATION_ROW_KEY_PREFIX};
use semio_framework_os_renderer_wgpu_shell_doc_open::{OpeningState, present_media_export_bytes_cancellable};
use semio_framework_os_renderer_wgpu_shell_dock::{DockInputIdentity, ShellDock, ShellDockState, dock_tab_path, os_shell_config_document, shell_sync_pill_text};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, FRAMEWORK_CHAT_PANEL_ID, FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX, FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, FRAMEWORK_MARKETPLACE_TAB_ID, FRAMEWORK_SYNC_PANEL_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, FRAMEWORK_TOOL_PANEL_TAB_PREFIX, SearchPaletteItem, SpacePanelState, SpawnedAppEntry, shell_offered_examples, take_spawned_entry};
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, TimeTravelFocus};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, ShellInputState, ShellPaletteKind};
use semio_framework_os_renderer_wgpu_shell_plugins::{PluginState, ShellExtensionLoadStatus, ShellExtensionProjection, flatten_panel_tab_leaves};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, DockTabNode, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, PrefsState, ResolvedCommand, TimeTravelVerb, command_address_stable_key, display_panel_body, dsl_value_as_json, program_fault_text};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, ShellDetached, SurfaceState, TutorialState};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::{ShellIoCompletion, ShellPoolFuture};
use semio_framework_os_renderer_wgpu_shell_sync::{SyncState, shell_sync_channel_owner};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, PanelProjection, WindowPaneChip, WindowState, panel_ui_scroll_records};
use infinite_world::world::begin_world3d_dynamic_retirement;
use semio_framework::{AppDefinition, IconName, PanelGroup, PanelTabDefinition, ViewModel};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::Lane;
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, SCENE_SURFACE_CAPACITY};
use semio_framework_os_renderer_wgpu_dock::DockState;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_scene_feeds::{NodeGraphSurface, TiledMapSurface};
use semio_framework_os_renderer_wgpu_scene_text_vfs::Board2dSurface;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_value::{DslValue, FromValue};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use store_sync::sync::ArtifactActorMsg;
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::{ActionDescriptor, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_INSPECTION_ID, InputState, Rect, Theme, UiButtonNode, UiControlNode, UiInputNode, UiNode, UiNumberStepperNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent, UtilityNode, WindowLayout};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_session__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⏪️time-travel/🦀️.rs"]
mod staged_session__time_travel;
pub use staged_session__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_session__dock_model_and_measures;
pub use staged_session__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_session__settings_and_dock_persistence;
pub use staged_session__settings_and_dock_persistence::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_session__theme_file_import_and_accessibility;
pub use staged_session__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🎬️shell-actions/🦀️.rs"]
mod staged_session__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_session__space_admin_and_settle;
pub use staged_session__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🐚️shell-parent/🦀️.rs"]
mod staged_session__shell_parent;
pub use staged_session__shell_parent::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_session__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/📎️local-folders/🦀️.rs"]
mod staged_session__local_folders;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/📨️command-dispatch/🦀️.rs"]
mod staged_session__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🔖️identity-and-types/🦀️.rs"]
mod staged_session__identity_and_types;
pub use staged_session__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🖱️pointer-input/🦀️.rs"]
mod staged_session__pointer_input;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_session__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_session__chrome_overlays_and_tour;
pub use staged_session__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🧩️session-and-topology/🦀️.rs"]
mod staged_session__session_and_topology;
pub use staged_session__session_and_topology::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_session__window_actions_and_commands;
pub use staged_session__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_session__chrome_paint_and_navbar;
pub use staged_session__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🧺️bundle/🦀️.rs"]
mod staged_session__bundle;
pub use staged_session__bundle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🧾️panel-records/🦀️.rs"]
mod staged_session__panel_records;
pub use staged_session__panel_records::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚶️session/🪟️tree-windows/🦀️.rs"]
mod staged_session__tree_windows;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
