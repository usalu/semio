//! 📂️ OpeningState component of the shell: document opening, import transfers and icon export.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log, directory_now_ms};
use semio_framework_os_renderer_wgpu_shell_chrome_state::ChromeState;
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_docs::DocumentState;
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_prefs::{PrefsState, dsl_value_as_json, settings_text_row};
use semio_framework_os_renderer_wgpu_shell_surfaces::{OpenedFile, PickedImport, ShellDetached};
use semio_framework_os_renderer_wgpu_shell_sync::{Phase, ShellHubDocumentSeed, ShellPreparedDocumentOpen, SyncState};
#[cfg(not(target_arch = "wasm32"))]
use protocol::os_directory::client::InstalledServiceContributionV1;
use semio_framework::{AppDefinition, IconName};
use semio_framework_async::CancelToken;
use semio_framework_os_kernel::os_directory::client::DirectoryClientError;
use semio_framework_os_kernel::os_directory::identity::Identity;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::{DocumentExecutionTargetLeaseFieldsV1, DocumentScope};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_host_base::host_io_call;
use semio_framework_os_renderer_wgpu_program_bridge::ProgramBridgeEntry;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::Label;
use semio_framework_value::DslValue;
use serde::Deserialize;
use std::collections::VecDeque;
use store_sync::sync::{ArtifactDocumentKey, PersistenceBinding};
use ui_wgpu::wgpu::{ActionDescriptor, UiButtonNode, UiNode, UiPresence, UiStackNode};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🧬️component/🦀️.rs"]
mod staged_doc_open__component;
pub use staged_doc_open__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_doc_open__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_doc_open__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_doc_open__theme_file_import_and_accessibility;
pub use staged_doc_open__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🎬️shell-actions/🦀️.rs"]
mod staged_doc_open__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_doc_open__space_admin_and_settle;
pub use staged_doc_open__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/📤️icon-export/🦀️.rs"]
mod staged_doc_open__icon_export;
pub use staged_doc_open__icon_export::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/📨️command-dispatch/🦀️.rs"]
mod staged_doc_open__command_dispatch;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_doc_open__storage_prefs_and_i18n;
pub use staged_doc_open__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🔖️identity-and-types/🦀️.rs"]
mod staged_doc_open__identity_and_types;
pub use staged_doc_open__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_doc_open__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_doc_open__chrome_overlays_and_tour;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_doc_open__chrome_paint_and_navbar;
pub use staged_doc_open__chrome_paint_and_navbar::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
