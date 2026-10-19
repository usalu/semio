//! 🥣️ Companion test bin of the wgpu renderer: documents, sync and directory (3).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate semio_framework_os_kernel as store_sync;
use semio_framework::kernel::IconRenderExportItem;
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, WorkerPool};
use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard;
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, mouse_pointer, paint_component_pointer_documents, pointer_interaction, tree_pointer_record};
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_icon_host::icon_export::IconExportFault;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_doc_open::{IconExportBatch, file_open_import_actions};
use semio_framework_os_renderer_wgpu_shell_dock::{PanelAnchorState, ShellDock};
use semio_framework_os_renderer_wgpu_shell_docs::{
    FRAMEWORK_SYNC_PANEL_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, SHELL_DOCUMENT_RETIREMENT_BYTES, SHELL_DOCUMENT_RETIREMENT_CAPACITY, SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES, SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_WORD_BITS,
    SHELL_DOCUMENT_RETIREMENT_WORDS, ShellDocumentRetirementRegistry, ShellDocumentRetirementSlot,
};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_plugins::plugin_modules_root_of;
use semio_framework_os_renderer_wgpu_shell_plugins::{ShellPluginInstall, ShellPluginInstallPhase};
use semio_framework_os_renderer_wgpu_shell_pointer::{CONTROL_ID, PointerCtx};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, claim_app_catalogue_fetch, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::OpenedFile;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_surfaces::{ShellIoCompletion, ShellPoolFuture};
use semio_framework_os_renderer_wgpu_shell_sync::ShellSyncOwner;
use semio_framework_os_renderer_wgpu_shell_view_chrome::ShellChromeChildCursor;
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{
    WindowActionsCtx, approx_sampled_timestamp_ms, decode_data_url, fallback_action_descriptor, ffmpeg_frame_extraction_args, open_artifact_relay_target, request_media_frames, shell_sync_owner_matches,
};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, section_fixtures};
use semio_framework_os_renderer_wgpu_test_support::shell_source;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_draw_contract::DrawList;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::Theme;
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use std::sync::atomic::Ordering;
use store_sync::PresenceWindowView;
use ui_contract::{SurfaceId, UI_DOCUMENT_LEASE_ALIASES, UI_DOCUMENT_LEASE_SLOTS, UiDocumentLease, UiFixedList, UiText};
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::{InputState, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;

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
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs"]
mod board_presence_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔄️sync-owner-lifetime/🦀️.rs"]
mod sync_owner_lifetime_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔄️detached-sync-task-manager/🦀️.rs"]
mod detached_sync_task_manager_tests;
