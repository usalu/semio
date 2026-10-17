//! 🥪️ Companion test bin of the wgpu renderer: documents, sync and directory (2).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_async::CancelToken;
use semio_framework_os_config::opening_config::mutations::{LOCAL_FOLDERS_CONFIG_SCHEMA, LocalFolderBinding, LocalFolderBindings, LocalFolderRef, LocalFoldersConfigMutation};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::DirectoryCommandOutcomeV1;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::DirectoryWsConnection;
#[cfg(any(not(target_arch = "wasm32"), not(target_arch = "wasm32")))]
use semio_framework_os_kernel::os_directory::client::TransportError;
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_kernel::os_directory::io::text::directory_command_sha256;
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::{SpaceArtifactCreationCatalogV1, SpaceArtifactCreationPhaseV1};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::{DocumentExecutionTargetLeaseFieldsV1, DocumentScope};
use semio_framework_os_kernel::os_directory::{DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility};
use semio_framework_os_renderer_wgpu_canvas2d_host::{identity, publish};
use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_renderer_wgpu_doors::directory_door::*;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard;
use semio_framework_os_renderer_wgpu_fixtures::shell::{boot_manifest, host_test_shell, paint_component_pointer_documents};
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_base::pump_renderer_io_sessions;
use semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::{HubSessionEvent, HubSessionPhase, HubSignInErrorCode, reduce_hub_session};
use semio_framework_os_renderer_wgpu_hub_contracts::space_browser;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::{FRAMEWORK_HUB_PANEL_ID, HubArtifactCatalogPhase, HubArtifactOpening, HubConnectionState, HubLink, HubSessionPresence, hub_artifact_creation_terminal};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_kernel_protocol::CANCELLED_KERNEL_TURNS;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramFixtureDocument;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_program_bridge::{load_resolved_program, load_wasm_plugins};
use semio_framework_os_renderer_wgpu_shell_actions::{
    ActionsCtx, SHELL_DOCUMENT_TRANSPORTS, SYNC_DOCUMENT_UNIDENTIFIED_CODE, document_bindings_admitted, parse_remote_backbone_uri, select_boot_program, should_checkpoint_before_detach, uncommitted_edit_count,
};
use semio_framework_os_renderer_wgpu_shell_async::chrome_now_ms;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::close_unowned_directory_dial;
use semio_framework_os_renderer_wgpu_shell_directory::{
    AGENT_BRIDGE_OFFER_SCOPE_REFRESH_MS, AgentBridgeOfferScopeOwnerV1, MAX_DIRECTORY_COMMAND_RESULTS, MAX_PENDING_DIRECTORY_COMMANDS, ShellDirectoryCommandQueueV1, ShellDirectoryCommandResultV1, ShellHubSignInAnswer, ShellHubTask, shell_actor,
};
use semio_framework_os_renderer_wgpu_shell_doc_open::{ShellDocumentOpenPhase, ShellDocumentOpening, default_persistence_bindings, document_execution_target_admitted};
use semio_framework_os_renderer_wgpu_shell_dock::shell_sync_pill_text;
use semio_framework_os_renderer_wgpu_shell_docs::{BackboneFolderVerb, FRAMEWORK_SYNC_PANEL_TAB_ID, LocalFolderIdentity, TEST_BACKBONE_FOLDER_HOST, backbone_folder_request, backbone_folder_write_answer, local_folder_reconnect_offer};
use semio_framework_os_renderer_wgpu_shell_history::{TEST_LOCAL_FOLDERS_LOG, local_folder_events_of_log, local_folders_log_after, local_folders_mutation_of_value, replay_local_folder_events};
use semio_framework_os_renderer_wgpu_shell_plugins::ShellPluginFault;
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_prefs::{
    DockTabNode, FOLDER_FORGET_ACTION, FOLDER_FORGET_CONTROL_ID, FOLDER_RECONNECT_ACTION, FOLDER_RECONNECT_CONTROL_ID, LocalFolderText, REPLAY_REFUSAL_V1, ReplayRefusalReasonV1, action_window_instance_id, folder_reconnect_band_plan,
    local_folder_name, local_folder_text,
};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, fold_history_patch};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_sync::{ShellHubDocumentV1, ShellHubProjectionV1, shell_hub_connection_summary_v1};
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, document_opening_banner_text};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{FOLDER_RECONNECT_STATUS_ID, OverlaysViewCtx};
use semio_framework_os_renderer_wgpu_shell_view_paint::shell_presence_label;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, presence_peer_rows_for_surface};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{
    AUTO_CHECKIN_EDIT_THRESHOLD, AUTO_CHECKIN_IDLE_MS, BootSelectionRequest, RemoteBackboneUri, ShellDocumentTransports, WindowActionsCtx, auto_checkin_should_fire, backbone_folder_read_answer, can_check_in, checkpoint_landed,
    directory_command_from_action, history_rows_oldest_first, open_artifact_relay_target, parse_persistence_binding, shell_sync_link_after_status, shell_sync_link_terminal, space_artifact_creation_replay_choice,
};
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_os_renderer_wgpu_test_support::shell_source;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use store_sync::PresencePeer;
use store_sync::sync::{ArtifactSyncStatus, DocumentLinkStatus, PersistenceBinding, RemoteState};
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitTarget, InputState, Rect, Theme, UiNode};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]
mod identity_directory_presence_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔗️hub-projection-workspace/🦀️.rs"]
mod hub_projection_workspace_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-local-folders/🦀️.rs"]
mod local_folders_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs"]
mod document_relay_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs"]
mod shell_boot_isolation_tests;
#[path = "../../../🧪️tests/📇️wgpu-directory-door/🦀️.rs"]
mod directory_door_tests;
