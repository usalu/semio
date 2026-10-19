//! 🐙️ The ShellState composer of the fifteen shell components.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, ShellChromeBuildState, ShellChromePresentState};
use semio_framework_os_renderer_wgpu_shell_commands::CommandState;
use semio_framework_os_renderer_wgpu_shell_directory::{AgentBridgeOfferScopeOwnerV1, DirectoryState, ShellDirectoryCommandQueueV1, shell_directory_request_base_url};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_shell_directory::shell_directory_transport;
use semio_framework_os_renderer_wgpu_shell_doc_open::OpeningState;
use semio_framework_os_renderer_wgpu_shell_dock::{DEFAULT_PANEL_WIDTH_PX, ShellDock, ShellDockState, TreeWindowScheduler};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, ShellDocumentRetirementRegistry};
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, prefs_get};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PresentedInputGeometry, RightClickState, ShellInputState};
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_prefs::PrefsState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, ShellSettlePump, SurfaceState, TutorialState};
use semio_framework_os_renderer_wgpu_shell_sync::SyncState;
use semio_framework_os_renderer_wgpu_shell_windows::WindowState;
use semio_framework::kernel::UiDirtyScope;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::ScopeOwner;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::identity::claimed_local_hub_credential;
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, SCENE_SURFACE_CAPACITY, ShellFindItems};
use semio_framework_os_renderer_wgpu_dock::DockState;
use semio_framework_os_renderer_wgpu_doors::directory_door::ShellDirectoryTransport;
use semio_framework_os_renderer_wgpu_hub_contracts::hub_sign_in::{HUB_CONNECTION_BOOK_STORAGE_KEY_V1, parse_hub_origin};
use semio_framework_os_renderer_wgpu_program_bridge::{ProgramBridgeEntry, resolve_plugin_host_config};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_services::{ComputePool, TokioHostRuntime};
use semio_framework_ui_locale::{Locale, Terminology};
use std::collections::{BTreeMap, HashMap, VecDeque};
use store_sync::sync::{ArtifactHost, DocumentLinkStatus};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_widget_kit::WidgetInteractionMaps;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🧬️composer/🦀️.rs"]
mod staged_state__composer;
pub use staged_state__composer::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_state__theme_file_import_and_accessibility;
pub use staged_state__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🔖️identity-and-types/🦀️.rs"]
mod staged_state__identity_and_types;
pub use staged_state__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/🧩️session-and-topology/🦀️.rs"]
mod staged_state__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🐙️state/📎️local-folders/🦀️.rs"]
mod staged_state__local_folders;
pub use staged_state__local_folders::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
