//! 🥣️ Companion test bin of the wgpu renderer: documents, sync and directory (3).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_os_renderer_wgpu_shell_doc_open::{IconExportBatch, file_open_import_actions};
use semio_framework_os_renderer_wgpu_shell_docs::{SHELL_DOCUMENT_RETIREMENT_BYTES, SHELL_DOCUMENT_RETIREMENT_CAPACITY, SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES, SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_WORDS, SHELL_DOCUMENT_RETIREMENT_WORD_BITS, ShellDocumentRetirementRegistry, ShellDocumentRetirementSlot};
use semio_framework_os_renderer_wgpu_shell_plugins::{ShellPluginInstall, ShellPluginInstallPhase};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_plugins::plugin_modules_root_of;
use semio_framework_os_renderer_wgpu_shell_pointer::{CONTROL_ID, PointerCtx};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, claim_app_catalogue_fetch};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::OpenedFile;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::{ShellIoCompletion, ShellPoolFuture};
use semio_framework_os_renderer_wgpu_shell_view_chrome::ShellChromeChildCursor;
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_window_actions::{WindowActionsCtx, decode_data_url, fallback_action_descriptor, open_artifact_relay_target, request_media_frames};
#[cfg(any(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_window_actions::{approx_sampled_timestamp_ms, ffmpeg_frame_extraction_args};
use semio_framework::kernel::IconRenderExportItem;
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, WorkerPool};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_icon_host::icon_export::IconExportFault;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use ui_contract::{SurfaceId, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS, UiDocumentLease, UiFixedList, UiText};
use ui_wgpu::wgpu::{DrawList, FontAtlas, InputState, Theme};

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs"]
mod tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"]
mod shell_pool_future_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-document-retirement/🦀️.rs"]
mod shell_document_retirement_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🛍️app-catalogue-attempt/🦀️.rs"]
mod app_catalogue_attempt_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/📤️wgpu-file-open-import/🦀️.rs"]
mod file_open_import_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-media-frames/🦀️.rs"]
mod media_frames_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⬇️wgpu-media-export-encoding/🦀️.rs"]
mod media_export_encoding_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-plugin-install/🦀️.rs"]
mod plugin_install_tests;


const SHELL_CRATE_FOLDERS: &[&str] = &["📖️shell-vocab", "⏳️shell-async", "🎚️shell-chrome-state", "🫧️shell-surfaces", "🧲️shell-plugins", "⏪️shell-history", "🎛️shell-prefs", "📒️shell-directory", "🔃️shell-sync", "📄️shell-docs", "📂️shell-doc-open", "🔲️shell-windows", "⚓️shell-dock", "🕹️shell-input", "🎹️shell-commands", "🐙️shell-state", "🚶️shell-session", "🪝️shell-window-actions", "🔀️shell-actions", "🖐️shell-pointer", "🖍️shell-view-paint", "🍱️shell-view-chrome", "🗨️shell-view-overlays", "🪀️shell-view-panels"];

pub(crate) fn shell_source() -> &'static str {
    static SOURCE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    SOURCE
        .get_or_init(|| {
            let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let mut roots = vec![manifest.join("../../../../..").join("🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")];
            for folder in SHELL_CRATE_FOLDERS {
                roots.push(manifest.join("../../..").join(folder).join("🦀️.rs"));
            }
            roots
                .iter()
                .map(|root| semio_framework_os_renderer_wgpu_test_support::read_module_tree_source(root).expect("the shell source tree is readable"))
                .collect::<String>()
        })
        .as_str()
}
