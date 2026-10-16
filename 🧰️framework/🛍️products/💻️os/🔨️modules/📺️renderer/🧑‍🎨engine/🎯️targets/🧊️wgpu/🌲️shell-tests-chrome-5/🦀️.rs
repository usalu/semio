//! 🌲️ Companion test bin of the wgpu renderer: chrome and views (5).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_dock as dock;
use infinite_world::world::World3dState;
use semio_framework::ViewModel;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_async::CancelToken;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::WorkerPool;
use semio_framework_os_config::opening_config::{UI_PREFERENCES_CONFIG_SCHEMA, UiAppearance as OsUiAppearance, UiLocale as OsUiLocale};
use semio_framework_os_config::opening_config::mutations::set_locale;
use semio_framework_os_renderer_wgpu_canvas2d_host::paint;
use semio_framework_os_renderer_wgpu_dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX;
use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, split_pane_shell, test_app};
use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_program_bridge::plugin_artifact_kind_activations;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_async::HOST_STORAGE_SNAPSHOT_MAX_BYTES;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_async::PrefsStore;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ShellChromeBuildState, appearance_id, format_keybinding_shortcut, shell_control_hotkey_badge, shell_shortcut_table, window_projection_surface_id};
use semio_framework_os_renderer_wgpu_shell_dock::{flowed_anchor_content_rect, retained_panel_chrome_item_width};
use semio_framework_os_renderer_wgpu_shell_docs::{FRAMEWORK_DISPLAY_LAYOUT_TAB_ID, FRAMEWORK_DISPLAY_WINDOWS_TAB_ID, FRAMEWORK_MARKETPLACE_TAB_ID, FRAMEWORK_TOOL_PANEL_TAB_PREFIX, WORLD_PROJECTION_TEMPLATES, resolve_theme_for_ids, should_auto_start_introduction};
use semio_framework_os_renderer_wgpu_shell_history::{HOST_STORAGE_KEY_CENSUS, HOST_STORAGE_VALUE_MAX_BYTES, HostStorageKeyKind, OS_SHELL_CONFIG_STORAGE_KEY, STORAGE_DOOR_OP, StorageDoorScope, StorageDoorVerb, decode_host_storage_snapshot, decode_storage_door_answer, encode_storage_door_request, host_storage_carries_key, host_storage_get, host_storage_remove, host_storage_set, seed_host_storage};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_history::{prefs_get_from, prefs_set_in};
use semio_framework_os_renderer_wgpu_shell_plugins::{ShellExtensionLoadStatus, ShellExtensionStoreRecord};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, pointer_press_belongs_to_shell_chrome};
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, ChromeGroupItem, DockTabNode, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, ShellConflictRow, UI_COMPUTE_WORKER_COUNT_STORAGE_KEY, UiPreferencesEventLog, decode_ui_preferences_event_log, encode_ui_preferences_event_log, replay_ui_preferences, ui_preferences_log_write};
use semio_framework_os_renderer_wgpu_shell_session::{FRAMEWORK_SETTINGS_PANEL_ID, MARKETPLACE_LOCAL_SECTION_ID, MARKETPLACE_WINDOW_NODE_BUDGET, SessionCtx, panel_ui_records, shell_navbar_title};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, ShellNavbarControl, UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX, panel_band, resolve_shell_navbar_control_width, shell_chrome_centered_band, shell_chrome_free_band, window_projection_rect};
use semio_framework_os_renderer_wgpu_shell_view_overlays::OverlaysViewCtx;
use semio_framework_os_renderer_wgpu_shell_view_paint::{SHELL_WINDOW_PAINT_OPPORTUNITIES, ShellChromeBandSpan, ShellNavbarWidthPolicy, WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES, flowed_anchor_tab_divider_y, flowed_anchor_tab_row_y, shell_surface_role_chip_text};
use semio_framework_os_renderer_wgpu_shell_view_panels::PanelsViewCtx;
use semio_framework_os_renderer_wgpu_shell_window_actions::WINDOW_PANE_BODY_WIDTH_PX;
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WORLD_PROJECTION_DEFAULT_TEMPLATE_ID, WORLD_PROJECTION_PANE_PARENT, WindowPaneChip};
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_ui_viewport::{Viewport3dOrthographicView, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use ui_contract::SurfaceId;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Rect, Theme, UiControlNode, UiNode, WindowMeasure};
use semio_framework_os_renderer_wgpu_test_support::shell_source;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🗄️browser-prefs-persistence/🦀️.rs"]
mod browser_prefs_persistence_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧭️wgpu-navbar-footer-parity/🦀️.rs"]
mod navbar_footer_parity_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🖥️wgpu-display-conflicts-marketplace/🦀️.rs"]
mod display_conflicts_marketplace_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs"]
mod settings_general_layout_tests;
