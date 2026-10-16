//! 🪝️ Shell use-cases: window actions, command bridge, refresh and boot.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log, instant_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, ThemeDocument, WINDOW_SEARCH_BODY_KEY, window_actions_surface_id, window_measures_surface_id, window_search_surface_id};
use semio_framework_os_renderer_wgpu_shell_commands::{CommandState, resolved_execute_args};
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::ShellInferenceRunner;
use semio_framework_os_renderer_wgpu_shell_doc_open::{OpenArtifactRelayTarget, OpeningState, request_file_open, wgpu_document_socket_surface};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_doc_open::PendingFileOpen;
use semio_framework_os_renderer_wgpu_shell_dock::{ShellDockState, tutorial_resolve_gesture_point};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, LocalFolderIdentity, note_shell_command_action};
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_docs::{BackboneFolderVerb, archive_digest, backbone_folder_call, backbone_folder_request};
use semio_framework_os_renderer_wgpu_shell_history::HistoryState;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_history::HostRouteFolder;
use semio_framework_os_renderer_wgpu_shell_input::ShellInputState;
use semio_framework_os_renderer_wgpu_shell_plugins::{PluginState, find_dialect_app, flatten_panel_tab_leaves};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_plugins::PendingExtensionInvocation;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, PrefsState, ReplayRefusalReasonV1, dsl_value_as_json, labels, with_chrome_prefs};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, app_owns_command, declare_boot_subphase, download_media_export, panel_state_from_view, program_space_index, resident_census, staged_arg_row, staged_number};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{PickedImport, SettleState, ShellRenderedRefresh, SurfaceState, TutorialCameraConverge, TutorialMode, TutorialPendingDocOp, TutorialRuntime, TutorialState, World3dComputeStatus, orbit_to_tutorial_camera, tutorial_apply_camera_pose, tutorial_capture_camera_pose};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellIoCompletion;
use semio_framework_os_renderer_wgpu_shell_sync::{ShellHubDocumentSeed, ShellSyncOwner, SyncState};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WINDOW_PANE_CHIP_PARENT, WindowState, panel_ui_scroll_records, window_host_event_action};
use infinite_world::world::{WorldInteractionIntent, enqueue_world3d_events};
#[cfg(not(target_arch = "wasm32"))]
use protocol::os_directory::client::InstalledServiceStatusV1;
use semio_framework::{AppDefinition, IconName, ViewModel};
use semio_framework::kernel::{UiDirtyScope, UiDirtySection};
use semio_framework_os_kernel::os_directory::{DirectoryCommand, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1;
use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;
use semio_framework_os_renderer_wgpu_dock::parse_path;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use store_sync::sync::{ArtifactActorMsg, ArtifactHost, ArtifactMailboxSender, DocumentLinkStatus, PersistenceBinding, RemoteState};
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, HitKind, Rect, Theme, UiButtonNode, UiFieldNode, UiNode, UiNumberStepperNode, UiPresence, UiRingNode, UiSelectItem, UiSelectNode, UiSliderNode, UiStackNode, UiTextNode, UiToggleNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_window_actions__dock_model_and_measures;
pub use staged_window_actions__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_window_actions__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🎓️tutorial-runtime/🦀️.rs"]
mod staged_window_actions__tutorial_runtime;
pub use staged_window_actions__tutorial_runtime::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_window_actions__theme_file_import_and_accessibility;
pub use staged_window_actions__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🎬️shell-actions/🦀️.rs"]
mod staged_window_actions__shell_actions;
pub use staged_window_actions__shell_actions::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_window_actions__space_admin_and_settle;
pub use staged_window_actions__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/📎️local-folders/🦀️.rs"]
mod staged_window_actions__local_folders;
pub use staged_window_actions__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/📨️command-dispatch/🦀️.rs"]
mod staged_window_actions__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_window_actions__storage_prefs_and_i18n;
pub use staged_window_actions__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🔖️identity-and-types/🦀️.rs"]
mod staged_window_actions__identity_and_types;
pub use staged_window_actions__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🖱️pointer-input/🦀️.rs"]
mod staged_window_actions__pointer_input;
pub use staged_window_actions__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_window_actions__chrome_overlays_and_tour;
pub use staged_window_actions__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧩️session-and-topology/🦀️.rs"]
mod staged_window_actions__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_window_actions__window_actions_and_commands;
pub use staged_window_actions__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_window_actions__chrome_paint_and_navbar;
pub use staged_window_actions__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧵️shell-async-runners/🦀️.rs"]
mod staged_window_actions__shell_async_runners;
pub use staged_window_actions__shell_async_runners::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧺️bundle/🦀️.rs"]
mod staged_window_actions__bundle;
pub use staged_window_actions__bundle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪝️window-actions/🧾️panel-records/🦀️.rs"]
mod staged_window_actions__panel_records;
pub use staged_window_actions__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
