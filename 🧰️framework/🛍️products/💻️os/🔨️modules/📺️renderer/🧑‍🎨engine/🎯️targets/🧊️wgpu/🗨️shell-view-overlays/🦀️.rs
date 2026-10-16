//! 🗨️ Shell overlay views: dialogs, context menus, tour, bands and hub workspace.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_shell_async::chrome_now_ms;
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogFieldKind, ChromeDialogRequest, ChromeDialogStop, ChromeState, ChromeTooltipHover, chrome_tooltip_ready, format_keybinding_shortcut};
use semio_framework_os_renderer_wgpu_shell_commands::CommandState;
use semio_framework_os_renderer_wgpu_shell_directory::{BAND_LABEL_PEER_EDITING_HISTORY, BAND_LABEL_PEER_EDITING_ROW, BAND_LABEL_PEER_EDITING_TARGET, DirectoryState, ShellBoardPresencePaint, board_presence_chip_width};
use semio_framework_os_renderer_wgpu_shell_doc_open::OpeningState;
use semio_framework_os_renderer_wgpu_shell_dock::ShellDockState;
use semio_framework_os_renderer_wgpu_shell_docs::DocumentState;
use semio_framework_os_renderer_wgpu_shell_history::HistoryState;
use semio_framework_os_renderer_wgpu_shell_input::{ChromeDialogPaintOp, ChromeDialogSemantics, ContextMenuItem, ContextMenuState, OverlayState, ShellInputState, ShellPaletteKind, TIME_TRAVEL_BAND_STATUS_ID, note_chrome_control_description, note_chrome_control_disabled, note_chrome_control_name, with_chrome_control_names};
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_pointer::{UI_INTRODUCTION_BACK_CONTROL_ID, UI_INTRODUCTION_NEXT_CONTROL_ID, UI_INTRODUCTION_SKIP_CONTROL_ID, UI_INTRODUCTION_VEIL_CONTROL_ID};
use semio_framework_os_renderer_wgpu_shell_prefs::{BAND_LABEL_ACCEPTED, BAND_LABEL_PREPARATION_PROGRESS, BAND_LABEL_PROCESSED, BAND_LABEL_PROGRESS, BAND_LABEL_REPLAY_FAULTED, BAND_LABEL_TARGET, BAND_LABEL_WORST, BandTongue, CHROME_ICON_TINY, ChromeGroupItem, LocalFolderText, PrefsState, TimeTravelVerb, band_label, history_reprojection_message, history_reprojection_running, local_folder_name, local_folder_path, local_folder_text, retained_chrome_group_item_width, time_travel_band_lines, time_travel_progress_key, time_travel_stage_key, time_travel_target_text};
use semio_framework_os_renderer_wgpu_shell_session::{SHELL_PALETTE_TINY_UI_SPACING, SessionCtx, ShellPalettePlanEntry};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState, TutorialState};
use semio_framework_os_renderer_wgpu_shell_sync::SyncState;
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeChildCursor, chrome_text_complete_step, close_chrome_overlay_glass_content, note_chrome_group_item, open_chrome_overlay_glass_content, render_retained_chrome_group_item_step};
use semio_framework_os_renderer_wgpu_shell_view_paint::{RetainedChromeGroupStep, chrome_icon, chrome_text, scene_chrome_labels, surface_fits_overlay, transient_notice_tone, window_pane_chip_rect};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WINDOW_PANE_CHIP_PARENT, WindowState};
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::render_ui_document_step;
use semio_framework_ui_locale::{Label, Locale, Terminology};
use semio_framework_value::DslValue;
use std::collections::HashMap;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, Rect, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow, Rgba, Theme, chrome_item_bg, chrome_item_text, paint_retained_glyph_step_flowed};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/⏪️time-travel/🦀️.rs"]
mod staged_view_overlays__time_travel;
pub use staged_view_overlays__time_travel::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_overlays__theme_file_import_and_accessibility;
pub use staged_view_overlays__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/💬️chrome-dialog-and-tooltips/🦀️.rs"]
mod staged_view_overlays__chrome_dialog_and_tooltips;
pub use staged_view_overlays__chrome_dialog_and_tooltips::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/📎️local-folders/🦀️.rs"]
mod staged_view_overlays__local_folders;
pub use staged_view_overlays__local_folders::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🔖️identity-and-types/🦀️.rs"]
mod staged_view_overlays__identity_and_types;
pub use staged_view_overlays__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_overlays__chrome_overlays_and_tour;
pub use staged_view_overlays__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_overlays__navbar_footer_and_overlays;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_overlays__chrome_paint_and_navbar;
pub use staged_view_overlays__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🗨️view-overlays/🧺️bundle/🦀️.rs"]
mod staged_view_overlays__bundle;
pub use staged_view_overlays__bundle::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
