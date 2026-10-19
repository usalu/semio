//! 🏙️ Companion test bin of the wgpu renderer: windows and panels (1).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{PanelGroup, PanelTabDefinition, ViewModel, ViewSessionIdentity};
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_renderer_wgpu_agent_elements::agent_bridge;
use semio_framework_os_renderer_wgpu_canvas2d_host::paint;
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, ui_document_close_pending_for};
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragPayload, DockDragState, DockDropZone, DockSide, MODE_DOCK_MOBILE_MAX_WIDTH_PX};
use semio_framework_os_renderer_wgpu_fixtures::shell::{actions_shell, engaged_pane_shell, host_test_apps, host_test_shell, split_pane_app, split_pane_shell, test_app};
use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection;
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{SceneChromeLabels, SceneEngineHosts};
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{DockSkeleton, DockTabSkeleton, DockUiState, SHELL_SHORTCUT_ROWS, shell_shortcut_table};
use semio_framework_os_renderer_wgpu_shell_commands::resolved_execute_args;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryHomeProjection;
use semio_framework_os_renderer_wgpu_shell_directory::FRAMEWORK_CHAT_FEED_ID;
use semio_framework_os_renderer_wgpu_shell_dock::{DEFAULT_PANEL_WIDTH_PX, PanelAnchorSnapshot, ShellDock, write_os_shell_config_layer_in};
use semio_framework_os_renderer_wgpu_shell_docs::{FRAMEWORK_CHAT_PANEL_ID, FRAMEWORK_COMMAND_CATEGORY_TAB_PREFIX, FRAMEWORK_MARKETPLACE_TAB_ID, FRAMEWORK_SYNC_PANEL_TAB_ID, FRAMEWORK_TASK_MANAGER_PANEL_ID, SpacePanelState};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, shell_keybinding_rows};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, WINDOW_UTILITY_RAIL_PARENT, is_reserved_shell_chord_in, pointer_press_belongs_to_shell_chrome, shell_shortcut_for, shell_shortcut_for_in};
use semio_framework_os_renderer_wgpu_shell_prefs::{
    ActiveSession, ChromeGroupItem, DockTabNode, FRAMEWORK_SETTINGS_CONFLICTS_TAB_ID, FRAMEWORK_SETTINGS_DEFAULT_APPS_TAB_ID, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_KEYBINDINGS_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID,
    command_address_stable_key, dsl_value_as_json, labels, retained_chrome_group_item_width,
};
use semio_framework_os_renderer_wgpu_shell_session::{
    DockTabMoveTarget, FRAMEWORK_CATEGORY_COMMAND_ID, FRAMEWORK_SETTINGS_PANEL_ID, SessionCtx, ShellShortcut, apply_dock_skeleton, dock_leaf_tree_unit_id, dock_skeleton_of, dock_skeletons_equal, mobile_panel_reconcile_path, move_tab_in_dock,
    panel_json, panel_state_from_view, panel_ui_records, read_os_shell_config_layer, same_session,
};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, anchor_panel_rect, shell_chrome_centered_band};
use semio_framework_os_renderer_wgpu_shell_view_overlays::OverlaysViewCtx;
use semio_framework_os_renderer_wgpu_shell_view_paint::{SHELL_WINDOW_PAINT_OPPORTUNITIES, ShellChromeBandSpan, window_pane_chip_rect};
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, ShellChromeFrameCursor, ShellChromeFramePhase};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_shell_window_actions::{WindowActionsCtx, tool_run_panel_reveals};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WindowPaneChip};
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use ui_contract::{SurfaceId, UiDocumentLease};
use semio_framework_ui_declaration::layout::{ActionDescriptor, FRAMEWORK_PANEL_TAB_HISTORY_ID, FRAMEWORK_PANEL_TAB_TOOL_RUN_ID, WindowLayout};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState, KeyAction, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::{UiNode, UiPresence};
use semio_framework_ui_declaration::utilities::UtilityNode;

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs"]
mod panel_anchor_model_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️wgpu-window-pane-chrome/🦀️.rs"]
mod window_pane_chrome_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs"]
mod tool_run_panel_tests;
