//! 🌴️ Companion test bin of the wgpu renderer: chrome and views (4).
//!
//! 🧭️ The tests of this bin are mounted from their place in the element trees; the namespace they were written against is rebuilt by the imports below.

#![cfg(test)]

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_contracts::{SHELL_FIND_ITEM_CAPACITY, ShellFindItem, ShellFindItems, try_push_find_item};
use semio_framework_os_renderer_wgpu_dock::{DockNode, DockStackTab, WindowSilhouette, push_window_silhouette_border};
use semio_framework_os_renderer_wgpu_fixtures::shell::split_pane_shell;
use semio_framework_os_renderer_wgpu_shell_actions::{ActionsCtx, staged_arg_value};
use semio_framework_os_renderer_wgpu_shell_async::{INTRODUCED_PULSE_PERIOD_MS, engagement_completion_suffix, engagement_ghost_accept_on_click, introduced_pulse_thickness, utility_subtree_has_active_path};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_shell_chrome_state::assert_shell_chrome_build_state_is_send;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{
    CHROME_TOOLTIP_DELAY_MS, ChromeDialogRequest, ChromeDialogStop, ChromeTooltipHover, RetainedContentFocus, SHELL_SHORTCUT_ROWS, ShellChromeBuildState, ThemeDocument, ThemeNumber, ThemePaintRef, chrome_tooltip_ready, format_keybinding_shortcut,
    shell_theme_document_base,
};
use semio_framework_os_renderer_wgpu_shell_input::{
    ChromeDialogPaintOp, SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY, SHELL_THEME_APPEARANCE_GROUPS, SHELL_THEME_EDITOR_ROW_HEIGHT, SHELL_THEME_EDITOR_SCROLL_ID, SHELL_THEME_EDITOR_SECTIONS, note_chrome_control_name, with_chrome_control_names,
};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, ShellEditVerb, shell_edit_verb_for};
use semio_framework_os_renderer_wgpu_shell_prefs::{
    ChromeGroupItem, DockTabNode, FRAMEWORK_SETTINGS_GENERAL_TAB_ID, FRAMEWORK_SETTINGS_THEME_TAB_ID, dsl_value_as_json, retained_chrome_group_item_width, set_active_theme_id, theme_from_document, theme_paint_editor_values,
};
use semio_framework_os_renderer_wgpu_shell_session::{FRAMEWORK_SETTINGS_PANEL_ID, SessionCtx, panel_ui_records};
use semio_framework_os_renderer_wgpu_shell_state::{SHELL_DEFAULT_MERGE_POLICY, ShellState};
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, render_retained_chrome_group_item_step, render_utility_node_step, render_utility_nodes};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{INTRODUCTION_INFO_BOX_GAP, OverlaysViewCtx, chrome_dialog_paint_ops, introduction_veil_bands, punch_introduction_cutout, resolve_introduction_placement};
use semio_framework_os_renderer_wgpu_shell_view_paint::RetainedChromeGroupStep;
use semio_framework_os_renderer_wgpu_shell_view_panels::{PanelsViewCtx, ShellChromeFrameCursor, ShellChromeFramePhase};
use semio_framework_os_renderer_wgpu_shell_window_actions::{custom_theme_id_for_label, delete_custom_theme, engagement_control_rows, merge_committed_args, staged_color_value};
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::RetainedNodeFocusKind;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger;
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_layout::layout::ActionDescriptor;
use semio_framework_ui_locale::{Label, Locale, LocalizedLabel, Terminology};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use ui_wgpu::wgpu::{HitKind, HitTarget, InputState, KeyAction, PointerModifiers, RetainedGlyphCursor, UiControlNode, UiFieldNode, UiNode, UiNumberStepperNode, UiPresence, UiTreeItemNode, WindowStackCorner};

#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-chrome-overlays-tour/🦀️.rs"]
mod chrome_overlays_tour_tests;
#[path = "../../../🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs"]
mod theme_editor_and_accessibility_tests;
