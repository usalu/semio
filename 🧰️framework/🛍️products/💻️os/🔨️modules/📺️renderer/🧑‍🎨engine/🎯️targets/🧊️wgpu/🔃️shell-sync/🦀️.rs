//! 🔃️ SyncState component of the shell: sync backbone, reseed and hub check-in.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log, directory_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::ChromeState;
use semio_framework_os_renderer_wgpu_shell_directory::{DirectoryState, ShellHubSignInAnswer, ShellHubTask};
use semio_framework_os_renderer_wgpu_shell_history::HistoryState;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, PrefsState, settings_text_row};
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellDetached;
use semio_framework::IconName;
use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::mint_directory_command_request_id;
use semio_framework_os_kernel::os_directory::client::DirectoryClient;
use semio_framework_os_kernel::os_directory::identity::Identity;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::identity::IdentityStatus;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use semio_framework_os_renderer_wgpu_contracts::ShellHubRemoteV1;
use semio_framework_os_renderer_wgpu_doors::directory_door::ShellDirectoryClient;
use semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::{HubSessionEvent, HubSignInErrorCode, reduce_hub_session};
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::{HubConnectionState, HubDocumentRemote, HubLink, HubSessionPresence};
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use store_sync::sync::{ArtifactDocumentKey, ArtifactEvent, ArtifactHost, ArtifactMailboxSender, ArtifactSyncStatus, DocumentLinkStatus, PersistenceBinding, RemoteState};
use ui_wgpu::wgpu::{ActionDescriptor, UiButtonNode, UiInputNode, UiNode, UiPresence, UiStackNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🧬️component/🦀️.rs"]
mod staged_sync__component;
pub use staged_sync__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_sync__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🎬️shell-actions/🦀️.rs"]
mod staged_sync__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/📎️local-folders/🦀️.rs"]
mod staged_sync__local_folders;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/📤️icon-export/🦀️.rs"]
mod staged_sync__icon_export;
pub use staged_sync__icon_export::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/📨️command-dispatch/🦀️.rs"]
mod staged_sync__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🔖️identity-and-types/🦀️.rs"]
mod staged_sync__identity_and_types;
pub use staged_sync__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_sync__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🔃️sync/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_sync__chrome_paint_and_navbar;
pub use staged_sync__chrome_paint_and_navbar::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
