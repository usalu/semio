//! 🔀️ Shell use-cases: action dispatch, document open and sync and directory pumps.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log, directory_now_ms, instant_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, ThemeDocument, window_pane_owner};
use semio_framework_os_renderer_wgpu_shell_commands::{CommandState, resolved_execute_args};
use semio_framework_os_renderer_wgpu_shell_directory::{DirectoryState, resolve_identity_env};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::{DirectoryHomeProjection, ShellDirectoryHomePublicationOutcome};
use semio_framework_os_renderer_wgpu_shell_doc_open::{IMPORT_TRANSFER_CANCEL_ACTION, OpeningState, ShellDocumentOpenAnswer, ShellDocumentOpenPhase, ShellDocumentOpenTarget, ShellDocumentOpening, default_persistence_bindings, request_file_open};
use semio_framework_os_renderer_wgpu_shell_dock::{ShellDockState, save_dock_skeleton_to_store, save_dock_ui_to_store};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, FRAMEWORK_SYNC_PANEL_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, SHELL_MERGE_POLICY_OPTIONS, note_shell_command_action, shell_command_for_control};
use semio_framework_os_renderer_wgpu_shell_history::HistoryState;
use semio_framework_os_renderer_wgpu_shell_input::ShellInputState;
use semio_framework_os_renderer_wgpu_shell_plugins::{DEFAULT_APP_NONE_VALUE, PluginState, ShellExtensionStoreRecord, flatten_panel_tab_leaves};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_plugins::extension_store_answer;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, DockTabNode, FOLDER_FORGET_ACTION, FOLDER_RECONNECT_ACTION, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, LocalFolderText, PrefsState, ReplayRefusalReasonV1, command_address_stable_key, dsl_value_as_json, history_refusal_notice, local_folder_path, local_folder_text, set_active_theme_id};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, app_owns_command, boot_app_role, declare_boot_subphase, download_media_export, fold_history_patch, panel_json, program_space_index, same_session, staged_number};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, ShellDetached, ShellSettleWatch, SurfaceState, TutorialState, World3dComputeStatus};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellIoCompletion;
use semio_framework_os_renderer_wgpu_shell_sync::{ShellPreparedDocumentOpen, ShellSyncChannel, SyncState, shell_hub_document_key};
use semio_framework_os_renderer_wgpu_shell_window_actions::{AUTO_CHECKIN_EDIT_THRESHOLD, AUTO_CHECKIN_IDLE_MS, BootSelectionRequest, KEYBINDING_UNOWNED_LABEL_DE, KEYBINDING_UNOWNED_LABEL_EN, RemoteBackboneUri, SHELL_DEFERRED_CHAIN_ROUNDS, SHELL_SETTLE_STALL_STEPS, SHELL_SETTLE_TERMINAL_DRIVES, SHELL_THEME_IMPORT_ACCEPT, SPACE_INDEX_DOCUMENT_ID, SYNC_COMMAND_REJECTED_NOTICE_CODE, ShellDocumentTransports, ShellSettleVerdict, WindowActionsCtx, WindowScopeInstanceV1, WindowScopeTargetKindV1, auto_checkin_should_fire, bind_wgpu_document_socket_surface, can_check_in, checkpoint_landed, custom_theme_id_for_label, custom_ui_driver_id, delete_custom_theme, directory_command_from_action, find_space_index_app, history_rows_oldest_first, local_command_rejection_notice, open_artifact_relay_target, operation_publication_refresh, parse_persistence_binding, project_boot_app_role, refresh_scope_label, route_document_backbone_effects, save_custom_theme_document, scope_action_to_window, seed_hub_document, settle_progress_signature, shell_sync_owner_matches, space_artifact_creation_replay_choice, staged_color_value, surface_role_apps, window_action_definition};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_window_actions::request_file_save;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_window_actions::pick_native_path_from_host;
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WindowState};
use semio_framework::{AppDefinition, ViewModel};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_os_kernel::os_directory::{DirectorySpaceKind, DirectorySpaceVisibility};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::DirectoryStreamMessage;
use semio_framework_os_kernel::os_directory::client::DirectoryClientError;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::{DirectoryBootstrapTransition, TransportError};
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1;
use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;
use semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::{HubConnection, HubConnectionKind, HubSessionEvent, HubSessionPhase, HubSignInErrorCode, hub_connection_id_for_origin, parse_hub_origin, reduce_hub_session, remove_hub_connection, select_hub_connection, upsert_hub_connection};
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::{HubArtifactCatalogPhase, HubArtifactOpening, action as hub_action, hub_artifact_creation_intent, hub_artifact_creation_terminal};
use semio_framework_os_renderer_wgpu_program_bridge::{ProgramBridgeEntry, resolve_playground_app_id, resolve_registry_plugin_id};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Locale;
use semio_framework_value::{DslValue, FromValue};
use serde_json::Value;
use std::collections::BTreeMap;
use store_sync::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactSyncStatus, DocumentLinkStatus, PersistenceBinding};
use semio_framework_ui_declaration::layout::ActionDescriptor;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_actions__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/⏪️time-travel/🦀️.rs"]
mod staged_actions__time_travel;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_actions__theme_file_import_and_accessibility;
pub use staged_actions__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🎬️shell-actions/🦀️.rs"]
mod staged_actions__shell_actions;
pub use staged_actions__shell_actions::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_actions__space_admin_and_settle;
pub use staged_actions__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📎️local-folders/🦀️.rs"]
mod staged_actions__local_folders;
pub use staged_actions__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/📨️command-dispatch/🦀️.rs"]
mod staged_actions__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🔖️identity-and-types/🦀️.rs"]
mod staged_actions__identity_and_types;
pub use staged_actions__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_actions__chrome_overlays_and_tour;
pub use staged_actions__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🧩️session-and-topology/🦀️.rs"]
mod staged_actions__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_actions__window_actions_and_commands;
pub use staged_actions__window_actions_and_commands::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_actions__chrome_paint_and_navbar;
pub use staged_actions__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🧺️bundle/🦀️.rs"]
mod staged_actions__bundle;
pub use staged_actions__bundle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔀️actions/🧾️panel-records/🦀️.rs"]
mod staged_actions__panel_records;
pub use staged_actions__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
