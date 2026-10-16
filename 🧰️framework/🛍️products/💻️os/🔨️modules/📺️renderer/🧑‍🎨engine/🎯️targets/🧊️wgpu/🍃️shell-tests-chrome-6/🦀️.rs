//! 🍃️ Companion test bin of the wgpu renderer: chrome and views (6).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_config::io::text::mutations::decode_ui_preferences_config_mutation_json;
use semio_framework_os_config::opening_config::{UI_PREFERENCES_CONFIG_SCHEMA, UiAppearance as OsUiAppearance, UiLocale as OsUiLocale, UiPreferences, apply_ui_preferences_config_mutation};
use semio_framework_os_config::opening_config::mutations::UiPreferencesConfigMutation;
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_renderer_wgpu_agent_elements::{agent_approvals, agent_bridge, agent_chat_panel, agent_presence};
use semio_framework_os_renderer_wgpu_dock::{DockRenderContext, DockState};
#[cfg(test)]
use semio_framework_os_renderer_wgpu_dock::DockStackTab;
use semio_framework_os_renderer_wgpu_fixtures::shell::{tour_introduction, tour_shell};
use semio_framework_os_renderer_wgpu_host_base::{HostAppearance, HostAppearancePreference, WgpuBootBrand, WgpuBootDescriptor, WgpuBootLocks, appearance_is_dark, apply_boot_descriptor, boot_brand, boot_descriptor, host_appearance_preference, host_locale, host_platform, host_platform_uses_meta, resolve_theme, set_host_appearance, set_host_introduction_suppressed, set_host_locale, set_host_platform, set_host_platform_kind, set_host_platform_uses_meta};
use semio_framework_os_renderer_wgpu_hub_workspace::space_administration;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_async::resolve_language_axes;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{CHROME_TOOLTIP_DELAY_MS, ChromeTooltipHover, RefusedGuestFault, ShellChromeBuildState, ShellTransientNotice, TRANSIENT_NOTICE_AUTO_DISMISS_MS, ThemeDocument, chrome_tooltip_ready, shell_theme_document_base};
use semio_framework_os_renderer_wgpu_shell_directory::{AGENT_CHAT_VISIBLE_ENTRIES, ShellSpaceAdministrationPhaseV1, shell_space_administration_status};
use semio_framework_os_renderer_wgpu_shell_dock::{NAMED_LAYOUT_STORE_FALLBACK_APP_ID, named_layout_events, shell_sync_pill_text, user_named_layouts};
use semio_framework_os_renderer_wgpu_shell_docs::{FRAMEWORK_CHAT_PANEL_ID, command_host_platform, reserved_shell_chords_v1, resolve_theme_for_ids, should_auto_start_introduction};
use semio_framework_os_renderer_wgpu_shell_history::{OS_SHELL_CONFIG_STORAGE_KEY, empty_os_shell_config};
#[cfg(all(not(target_arch = "wasm32"), any(test, feature = "testkit")))]
use semio_framework_os_renderer_wgpu_shell_history::FilePrefsStore;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_history::{prefs_get_from, prefs_set_in};
use semio_framework_os_renderer_wgpu_shell_pointer::{ShellEditVerb, shell_edit_verb_for};
use semio_framework_os_renderer_wgpu_shell_prefs::{CHROME_ICON_TINY, COMMAND_REJECTED_FAULT_CODE, ChromeGroupItem, UiPreferencesEventLog, VIEWER_READ_ONLY_FAULT_CODE, classify_dispatch_fault_notice, decode_ui_preferences_event_log, encode_ui_preferences_event_log, env_lock, labels, project_chrome_prefs, replay_ui_preferences, resolve_appearance_id, resolve_locale_id, retained_chrome_group_item_width};
#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
use semio_framework_os_renderer_wgpu_shell_prefs::set_active_ui_layout;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_prefs::{active_theme_id, active_ui_layout, custom_theme_ids, shell_pref_locks};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, framework_display_tabs, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_sync::{ShellSyncPill, ShellSyncRemote};
use semio_framework_os_renderer_wgpu_shell_view_chrome::{UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX, render_retained_chrome_group_item_step, transient_notice_rect};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{AgentApprovalPaintOp, OverlaysViewCtx};
use semio_framework_os_renderer_wgpu_shell_view_paint::{RetainedChromeGroupStep, TRANSIENT_NOTICE_TOP_GAP, chrome_text_step, transient_notice_close_rect, transient_notice_tone};
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, TRANSIENT_NOTICE_STATUS_ID};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{WindowActionsCtx, delete_custom_theme, save_custom_theme_document, tutorial_apply_ui_snapshot};
use semio_framework_ui_declaration::ui::{UiControlNode, UiNode, UiSelectNode, UiTreeItemNode};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_value::DslValue;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use ui_wgpu::wgpu::{HitKind, InputState, KeyAction, PointerModifiers, RetainedGlyphCursor, RetainedGlyphStep};
use semio_framework_os_renderer_wgpu_test_support::shell_source;

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
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs"]
mod appearance_tour_and_footer_pill_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🚪️wgpu-host-door-remainder/🦀️.rs"]
mod host_door_remainder_tests;
