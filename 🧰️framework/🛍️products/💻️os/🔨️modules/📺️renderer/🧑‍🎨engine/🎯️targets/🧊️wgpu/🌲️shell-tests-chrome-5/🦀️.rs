//! 🌲️ Companion test bin of the wgpu renderer: chrome and views (5).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

use semio_framework_os_renderer_wgpu_shell_async::HOST_STORAGE_SNAPSHOT_MAX_BYTES;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_async::PrefsStore;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ShellChromeBuildState, appearance_id};
use semio_framework_os_renderer_wgpu_shell_docs::should_auto_start_introduction;
use semio_framework_os_renderer_wgpu_shell_history::{HOST_STORAGE_KEY_CENSUS, HOST_STORAGE_VALUE_MAX_BYTES, HostStorageKeyKind, OS_SHELL_CONFIG_STORAGE_KEY, STORAGE_DOOR_OP, StorageDoorScope, StorageDoorVerb, decode_host_storage_snapshot, decode_storage_door_answer, encode_storage_door_request, host_storage_carries_key, host_storage_get, host_storage_remove, host_storage_set, seed_host_storage};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_history::{prefs_get_from, prefs_set_in};
use semio_framework_os_renderer_wgpu_shell_prefs::{UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, UiPreferencesEventLog, decode_ui_preferences_event_log, encode_ui_preferences_event_log, replay_ui_preferences, ui_preferences_log_write};
use semio_framework_os_renderer_wgpu_shell_view_chrome::UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX;
use semio_framework_os_config::opening_config::{UI_PREFERENCES_CONFIG_SCHEMA, UiAppearance as OsUiAppearance, UiLocale as OsUiLocale};
use semio_framework_os_config::opening_config::mutations::set_locale;
use serde_json::Value;
use std::collections::HashMap;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs"]
mod browser_prefs_persistence_tests;


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
