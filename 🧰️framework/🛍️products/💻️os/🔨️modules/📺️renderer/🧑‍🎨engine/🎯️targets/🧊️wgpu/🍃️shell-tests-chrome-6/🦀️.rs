//! 🍃️ Companion test bin of the wgpu renderer: chrome and views (6).
//!
//! 🧭️ The Shell tests of this bin are mounted from their place in the Shell element tree; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_async::resolve_language_axes;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{CHROME_TOOLTIP_DELAY_MS, ChromeTooltipHover, RefusedGuestFault, ShellChromeBuildState, ShellTransientNotice, TRANSIENT_NOTICE_AUTO_DISMISS_MS, ThemeDocument, chrome_tooltip_ready, shell_theme_document_base};
use semio_framework_os_renderer_wgpu_shell_directory::{AGENT_CHAT_VISIBLE_ENTRIES, ShellSpaceAdministrationPhaseV1, shell_space_administration_status};
use semio_framework_os_renderer_wgpu_shell_docs::{FRAMEWORK_CHAT_PANEL_ID, resolve_theme_for_ids};
use semio_framework_os_renderer_wgpu_shell_history::OS_SHELL_CONFIG_STORAGE_KEY;
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_history::FilePrefsStore;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_history::{prefs_get_from, prefs_set_in};
use semio_framework_os_renderer_wgpu_shell_prefs::{CHROME_ICON_TINY, COMMAND_REJECTED_FAULT_CODE, ChromeGroupItem, UiPreferencesEventLog, VIEWER_READ_ONLY_FAULT_CODE, classify_dispatch_fault_notice, decode_ui_preferences_event_log, encode_ui_preferences_event_log, env_lock, labels, project_chrome_prefs, replay_ui_preferences, resolve_locale_id, retained_chrome_group_item_width};
#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_shell_prefs::set_active_ui_layout;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_prefs::{active_theme_id, active_ui_layout, custom_theme_ids, shell_pref_locks};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, framework_display_tabs, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX, render_retained_chrome_group_item_step, transient_notice_rect};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{AgentApprovalPaintOp, OverlaysViewCtx};
use semio_framework_os_renderer_wgpu_shell_view_paint::{RetainedChromeGroupStep, TRANSIENT_NOTICE_TOP_GAP, chrome_text_step, transient_notice_close_rect, transient_notice_tone};
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, TRANSIENT_NOTICE_STATUS_ID};
use semio_framework_os_renderer_wgpu_shell_window_actions::{delete_custom_theme, save_custom_theme_document};
use semio_framework_os_config::io::text::mutations::decode_ui_preferences_config_mutation_json;
use semio_framework_os_config::opening_config::{UI_PREFERENCES_CONFIG_SCHEMA, UiLocale as OsUiLocale, UiPreferences};
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_renderer_wgpu_agent_elements::{agent_approvals, agent_bridge, agent_chat_panel, agent_presence};
use semio_framework_os_renderer_wgpu_dock::{DockRenderContext, DockState};
#[cfg(test)]
use semio_framework_os_renderer_wgpu_dock::DockStackTab;
use semio_framework_os_renderer_wgpu_host_base::{WgpuBootDescriptor, WgpuBootLocks, apply_boot_descriptor, boot_descriptor, host_locale, resolve_theme, set_host_locale};
use semio_framework_os_renderer_wgpu_hub_workspace::space_administration;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, IconAtlas, InputState, Rect, RetainedGlyphCursor, RetainedGlyphStep, Rgba, Theme, UiControlNode, UiNode, UiSelectNode, UiTreeItemNode};
use semio_framework_os_renderer_wgpu_dock as dock;

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs"]
mod driver_editor_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs"]
mod retained_chrome_text_laws;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-ui-prefs-themes-i18n/🦀️.rs"]
mod ui_prefs_themes_i18n_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-agent-overlays/🦀️.rs"]
mod agent_overlays_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧪️wgpu-fault-notices/🦀️.rs"]
mod fault_notices_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/📐️wgpu-dpi-logical-units/🦀️.rs"]
mod dpi_logical_units_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/💓️chrome-maintenance-pressure/🦀️.rs"]
mod chrome_maintenance_pressure_tests;


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
