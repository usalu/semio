//! 🌆️ Companion test bin of the wgpu renderer: windows and panels (2).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use semio_framework::{AppDefinition, ViewModel};
use semio_framework_os_renderer_wgpu_contracts::{AdmittedSurfaceMap, ui_document_close_pending_for};
use semio_framework_os_renderer_wgpu_dock as dock;
use semio_framework_os_renderer_wgpu_dock::{DockNode, DockRenderContext, DockStackTab, DockState};
use semio_framework_os_renderer_wgpu_engine_attach::engine_surface_attach;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard;
use semio_framework_os_renderer_wgpu_fixtures::shell::{actions_shell, paint_component_pointer_documents, split_pane_shell, test_app, tree_pointer_record};
use semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions;
#[cfg(not(target_os = "wasi"))]
use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_wgpu_program_bridge::window_measures_from_section;
use semio_framework_os_renderer_wgpu_scene_feeds::{NodeGraphSurface, TiledMapSurface};
use semio_framework_os_renderer_wgpu_scene_text_vfs::Board2dSurface;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_async::window_measures_owner;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{CHROME_TOOLTIP_DELAY_MS, ChromeTooltipHover, window_actions_surface_id, window_measures_surface_id, window_search_surface_id};
use semio_framework_os_renderer_wgpu_shell_commands::{filter_search_possibles, normalize_engagement_action_text};
use semio_framework_os_renderer_wgpu_shell_dock::{PanelAnchorState, ShellDock, TREE_WINDOW_REPORT_DEBOUNCE_MS, TreeWindowScheduler, tutorial_resolve_gesture_point};
use semio_framework_os_renderer_wgpu_shell_docs::shell_command_for_control;
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PointerCapture, PointerHitOwner};
use semio_framework_os_renderer_wgpu_shell_pointer::PointerCtx;
use semio_framework_os_renderer_wgpu_shell_prefs::{ActiveSession, DockTabNode, dsl_value_as_json, labels};
use semio_framework_os_renderer_wgpu_shell_session::{SessionCtx, mirror_engine_surface_states, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{TutorialCameraConverge, TutorialMode, TutorialRuntime, orbit_to_tutorial_camera, tutorial_camera_to_orbit};
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor};
use semio_framework_os_renderer_wgpu_shell_view_paint::{SHELL_WINDOW_PAINT_OPPORTUNITIES, tutorial_scrub_progress, tutorial_scrub_target_ms};
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, ShellChromeFrameCursor};
use semio_framework_os_renderer_wgpu_shell_window_actions::{
    FRAMEWORK_RESERVED_ACTION_IDS, KEYBINDING_UNOWNED_CODE, WindowActionsCtx, scope_action_to_window, tutorial_advance_playhead, tutorial_apply_ui_change_to_shell, tutorial_apply_ui_snapshot, tutorial_camera_pose_close, tutorial_capture_ui_snapshot,
};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WindowPaneChip, panel_ui_scroll_records, section_fixtures, window_measures_overlay_records};
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::HashMap;
use semio_framework_ui_wgpu_light::tree_window::TreeWindowRequest;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, InputState, KeyAction, PointerModifiers};
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_ui_declaration::layout::{WindowEngagement, WindowMeasure, WindowStackCorner};
use winit::window::Window;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️tree-windows/🦀️.rs"]
mod tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🪟️action-window-scope/🦀️.rs"]
mod action_window_scope_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs"]
mod tutorial_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs"]
mod window_actions_search_pane_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs"]
mod window_measures_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🧲️engine-surface-retention/🦀️.rs"]
mod engine_surface_retention_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🛟️panel-window-reservation/🦀️.rs"]
mod panel_window_reservation_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🛰️wgpu-dock-close-reopen-journals/🦀️.rs"]
mod dock_close_reopen_journal_tests;
