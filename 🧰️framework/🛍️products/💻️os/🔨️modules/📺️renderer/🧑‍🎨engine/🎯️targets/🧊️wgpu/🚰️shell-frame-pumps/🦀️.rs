//! 🚰️ Shell frame pumps: directory events, media actions, retained document opening, auto check-in, operation publications and local-folder reattach.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚰️frame-pumps`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate. Split from `shell-actions`, which this crate builds on.

use semio_renderer_wgpu_shell_actions::*;
use semio_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_renderer_wgpu_shell_chrome_state::ChromeState;
use semio_renderer_wgpu_shell_commands::CommandState;
use semio_renderer_wgpu_shell_directory::DirectoryState;
#[cfg(not(target_arch = "wasm32"))]
use semio_renderer_wgpu_shell_directory::{DirectoryHomeProjection, ShellDirectoryHomePublicationOutcome};
use semio_renderer_wgpu_shell_doc_open::{OpeningState, ShellDocumentOpenAnswer, ShellDocumentOpenPhase};
use semio_renderer_wgpu_shell_dock::ShellDockState;
use semio_renderer_wgpu_shell_docs::DocumentState;
use semio_renderer_wgpu_shell_history::HistoryState;
use semio_renderer_wgpu_shell_input::ShellInputState;
use semio_renderer_wgpu_shell_plugins::PluginState;
use semio_renderer_wgpu_shell_prefs::PrefsState;
use semio_renderer_wgpu_shell_session::SessionCtx;
use semio_renderer_wgpu_shell_state::ShellState;
use semio_renderer_wgpu_shell_surfaces::{SettleState, ShellDetached, SurfaceState, TutorialState};
use semio_renderer_wgpu_shell_sync::SyncState;
use semio_renderer_wgpu_shell_window_actions::{AUTO_CHECKIN_EDIT_THRESHOLD, AUTO_CHECKIN_IDLE_MS, WindowActionsCtx, auto_checkin_should_fire, can_check_in, operation_publication_refresh};
use semio_renderer_wgpu_shell_windows::WindowState;
use semio_framework::kernel::UiDirtyScope;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_kernel::os_directory::DirectoryStreamMessage;
use semio_os_kernel::os_directory::client::DirectoryClientError;
#[cfg(not(target_arch = "wasm32"))]
use semio_os_kernel::os_directory::client::{DirectoryBootstrapTransition, TransportError};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚰️frame-pumps/🧭️pumps/🦀️.rs"]
mod staged_frame_pumps__pumps;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🚰️frame-pumps/🧺️bundle/🦀️.rs"]
mod staged_frame_pumps__bundle;
pub use staged_frame_pumps__bundle::*;
