//! 🍱️ Shell chrome views: navbar, footer, panels, tooltips and tutorial bar.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_actions::ActionsCtx;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_async::{chrome_group_border, utility_subtree_has_active_path};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeState, ChromeTooltipHover, ShellChromeBuildState, ShellTransientNotice, chrome_tooltip_ready, shell_control_hotkey_badge, window_measures_surface_id, window_projection_surface_id};
use semio_framework_os_renderer_wgpu_shell_commands::CommandState;
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_doc_open::{OpeningState, ShellDocumentOpenPhase, ShellDocumentOpening};
use semio_framework_os_renderer_wgpu_shell_dock::{PANEL_CHROME_GRIP_SIZE, PANEL_CHROME_ICON_TINY, ShellDockState, retained_panel_chrome_item_width};
use semio_framework_os_renderer_wgpu_shell_docs::{DocumentState, ShellExampleRow, note_shell_command_action};
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, SHELL_CHROME_IO_FIELD_BYTES};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PresentedInputCandidateWitness, PresentedInputGeometry, RightClickState, ShellInputState, ShellPaletteKind, note_chrome_control_name, panel_tab_introduction_element_id, panel_tab_scroll_id, tree_hit_item_id, with_chrome_control_names};
use semio_framework_os_renderer_wgpu_shell_plugins::{PluginState, ShellPluginInstallPhase};
use semio_framework_os_renderer_wgpu_shell_pointer::{PointerCtx, WINDOW_UTILITY_RAIL_PARENT, dock_tab_select_window_id, panel_resize_anchor, tree_row_has_chevron};
use semio_framework_os_renderer_wgpu_shell_prefs::{CHROME_ICON_TINY, ChromeGroupItem, DockTabNode, PrefsState, labels, retained_chrome_group_item_width, stored_field_get, stored_field_set};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_shell_prefs::measure_chrome_group_item;
use semio_framework_os_renderer_wgpu_shell_session::{DockTabMoveTarget, SessionCtx, mobile_panel_reconcile_path, panel_state_from_view, shell_navbar_title};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState, TutorialMode, TutorialState, World3dComputeStatus, tutorial_bar_height, world3d_compute_status};
use semio_framework_os_renderer_wgpu_shell_sync::SyncState;
use semio_framework_os_renderer_wgpu_shell_view_paint::{ANCHOR_COLUMN_ORDER, ChromePanelSafeArea, DOCUMENT_OPEN_CANCEL_CONTROL_ID, DOCUMENT_OPEN_STEPS, LOGO_UNTINTED, PLUGIN_INSTALL_CANCEL_CONTROL_ID, PLUGIN_INSTALL_TOP_GAP, RetainedChromeGroupStep, SHELL_NAVBAR_LOGO_UI_SPACING, SHELL_NAVBAR_SESSIONLESS_TITLE, SHELL_WINDOW_PAINT_OPPORTUNITIES, SafeAreaYield, ShellChromeBandSpan, ShellNavbarWidthPolicy, ShellUtilityPath, TAB_INSERT_PREVIEW_WIDTH_PX, TRANSIENT_NOTICE_CLOSE_CONTROL_ID, TRANSIENT_NOTICE_MAX_WIDTH, TRANSIENT_NOTICE_TOP_GAP, WORLD_PROJECTION_PANE_PAINT_OPPORTUNITIES, World3dStatusPill, chrome_icon, chrome_panel_axis_reserve, chrome_text, chrome_text_step, floating_panel_width, flowed_anchor_tab_divider_y, flowed_anchor_tab_row_y, plugin_install_action_rect, plugin_install_banner_text, rects_overlap, render_utility_section_divider, scene_chrome_labels, shell_presence_label, shell_surface_role_chip_text, surface_fits_overlay, surface_overlay_row_box, transient_notice_close_rect, transient_notice_tone, tutorial_scrub_progress, tutorial_scrub_target_ms, utility_at_path, utility_node_label, window_pane_chip_rect, world3d_status_is_visible};
use semio_framework_os_renderer_wgpu_shell_window_actions::{WINDOW_PANE_BODY_WIDTH_PX, WindowActionsCtx, surface_role_apps};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WORLD_PROJECTION_PANE_PARENT, WindowPaneChip, WindowState};
use semio_framework::AppDefinition;
use semio_framework_os_renderer_wgpu_contracts::ShellFindItem;
use semio_framework_os_renderer_wgpu_dock::{DockDragKind, DockDragPayload, parse_path};
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::vfs_selection_for_click;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, render_ui_document_step};
use semio_framework_ui_locale::{Locale, Terminology};
use std::collections::HashMap;
use store_sync::PresencePeer;
use ui_contract::UiText;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::{DragAxis, HitKind, HitTarget, InputState};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_theme::{Level, Rgba, Theme};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_paint::{RetainedGlyphCursor, RetainedGlyphStep};
use semio_framework_ui_declaration::utilities::UtilityNode;
use semio_framework_ui_wgpu_layout::chrome::{chrome_item_bg, chrome_item_text, push_chrome_group_border};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_view_chrome__dock_model_and_measures;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_chrome__theme_file_import_and_accessibility;
pub use staged_view_chrome__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_view_chrome__space_admin_and_settle;
pub use staged_view_chrome__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🖱️pointer-input/🦀️.rs"]
mod staged_view_chrome__pointer_input;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_chrome__chrome_frame_and_main_window;
pub use staged_view_chrome__chrome_frame_and_main_window::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_chrome__chrome_overlays_and_tour;
pub use staged_view_chrome__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_chrome__navbar_footer_and_overlays;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_chrome__chrome_paint_and_navbar;
pub use staged_view_chrome__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🧺️bundle/🦀️.rs"]
mod staged_view_chrome__bundle;
pub use staged_view_chrome__bundle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🍱️view-chrome/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_view_chrome__storage_prefs_and_i18n;
pub use staged_view_chrome__storage_prefs_and_i18n::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
