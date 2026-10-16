//! 📒️ DirectoryState component of the shell: identity, directory, hub workspace and presence.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as protocol;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log, directory_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::ChromeState;
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, prefs_set};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, BandTongue, PrefsState, band_label, settings_text_row};
use semio_framework_os_renderer_wgpu_shell_surfaces::SurfaceState;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::ShellPoolFuture;
#[cfg(not(target_arch = "wasm32"))]
use protocol::os_directory::client::{InstalledServiceContributionV1, InstalledServiceDriverV1, InstalledServiceStatusV1, InstalledServiceTurnV1};
use semio_framework::{AppDefinition, IconName, ViewModel, ViewSessionIdentity};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, WorkerPool};
use semio_framework_os_kernel::os_directory::{DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryCommandResultV1, DirectorySessionAuthorityV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationInviteRowV1, DirectorySpaceAdministrationMemberRowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, mint_directory_command_request_id};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::DirectoryStreamMessage;
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryClientError, LocalHubCredential};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::{CanonicalDirectoryEventPageV1, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, DirectoryStream, DirectoryStreamTurn, DirectoryWsConnection, TransportError};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1;
use semio_framework_os_kernel::os_directory::identity::{Identity, IdentityEnv, actor_id};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::identity::{IdentityOutcome, restore_claimed};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::DocumentScope;
use semio_framework_os_kernel::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1;
use semio_framework_os_renderer_wgpu_doors::directory_door::{ShellDirectoryClient, ShellDirectoryTransport};
use semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::{HUB_CONNECTION_BOOK_STORAGE_KEY_V1, HubSessionEvent, HubSignInClientClass, HubSignInCredential, HubSignInErrorCode, reduce_hub_session, serialize_hub_connection_book};
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::{HubArtifactCatalogPhase, HubArtifactCreation, HubArtifactCreationState, HubArtifactOpening, hub_artifact_creation_intent};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Label, Locale, Terminology};
use semio_framework_value::DslValue;
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, VecDeque};
use store_sync::{PresencePeer, PresenceWindowView};
use ui_wgpu::wgpu::{ActionDescriptor, Rect, Theme, UiButtonNode, UiInputNode, UiNode, UiPresence, UiStackNode, UiTextNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧬️component/🦀️.rs"]
mod staged_directory__component;
pub use staged_directory__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/⏪️time-travel/🦀️.rs"]
mod staged_directory__time_travel;
pub use staged_directory__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_directory__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🎬️shell-actions/🦀️.rs"]
mod staged_directory__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_directory__space_admin_and_settle;
pub use staged_directory__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_directory__chrome_dialog_and_tooltips;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/📨️command-dispatch/🦀️.rs"]
mod staged_directory__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_directory__storage_prefs_and_i18n;
pub use staged_directory__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🔖️identity-and-types/🦀️.rs"]
mod staged_directory__identity_and_types;
pub use staged_directory__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_directory__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_directory__chrome_overlays_and_tour;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧩️session-and-topology/🦀️.rs"]
mod staged_directory__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_directory__navbar_footer_and_overlays;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_directory__chrome_paint_and_navbar;
pub use staged_directory__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧵️shell-async-runners/🦀️.rs"]
mod staged_directory__shell_async_runners;
pub use staged_directory__shell_async_runners::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📒️directory/🧾️panel-records/🦀️.rs"]
mod staged_directory__panel_records;
pub use staged_directory__panel_records::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
