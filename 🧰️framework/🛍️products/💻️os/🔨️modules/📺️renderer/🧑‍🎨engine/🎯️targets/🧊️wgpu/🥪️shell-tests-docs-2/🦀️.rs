//! 🥪️ Companion test bin of the wgpu renderer: documents, sync and directory (2).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_actions::{should_checkpoint_before_detach, uncommitted_edit_count};
use semio_framework_os_renderer_wgpu_shell_directory::{MAX_DIRECTORY_COMMAND_RESULTS, MAX_PENDING_DIRECTORY_COMMANDS, ShellDirectoryCommandQueueV1, ShellDirectoryCommandResultV1, shell_actor};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::close_unowned_directory_dial;
use semio_framework_os_renderer_wgpu_shell_doc_open::default_persistence_bindings;
use semio_framework_os_renderer_wgpu_shell_dock::shell_sync_pill_text;
use semio_framework_os_renderer_wgpu_shell_session::fold_history_patch;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_panels::presence_peer_rows_for_surface;
use semio_framework_os_renderer_wgpu_shell_window_actions::{AUTO_CHECKIN_EDIT_THRESHOLD, AUTO_CHECKIN_IDLE_MS, auto_checkin_should_fire, can_check_in, checkpoint_landed, directory_command_from_action, open_artifact_relay_target};
use semio_framework_os_kernel::os_directory::{DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility};
#[cfg(all(test, not(target_arch = "wasm32")))]
use semio_framework_os_kernel::os_directory::DirectoryCommandOutcomeV1;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::{DirectoryWsConnection, TransportError};
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_kernel::os_directory::io::text::directory_command_sha256;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use store_sync::PresencePeer;
use store_sync::sync::{ArtifactSyncStatus, PersistenceBinding, RemoteState};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs"]
mod identity_directory_presence_tests;
