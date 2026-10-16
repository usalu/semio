//! 🪀️ Shell panel views: main window, footer, accessibility nodes and icon export.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as store_sync;
use semio_framework_os_renderer_wgpu_shell_async::{chrome_now_ms, debug_log};
use semio_framework_os_renderer_wgpu_shell_chrome_state::{ChromeDialogStop, ChromeState, shell_control_hotkey_badge, transient_notice_expired, window_actions_surface_id, window_search_surface_id};
use semio_framework_os_renderer_wgpu_shell_commands::CommandState;
use semio_framework_os_renderer_wgpu_shell_directory::DirectoryState;
use semio_framework_os_renderer_wgpu_shell_doc_open::{OpeningState, present_media_export_bytes_cancellable};
use semio_framework_os_renderer_wgpu_shell_dock::{ShellDockState, retained_panel_chrome_item_width};
use semio_framework_os_renderer_wgpu_shell_docs::DocumentState;
use semio_framework_os_renderer_wgpu_shell_history::{HistoryState, SHELL_CHROME_IO_FIELD_BYTES};
use semio_framework_os_renderer_wgpu_shell_input::{OverlayState, PresentedInputCandidateWitness, SHELL_CHROME_ACCESSIBLE_NAME_CAPACITY, ShellInputState, ShellPaletteKind, humanize_control_id, note_chrome_control_name, panel_tab_introduction_element_id, tree_hit_item_id, with_chrome_control_names};
use semio_framework_os_renderer_wgpu_shell_plugins::PluginState;
use semio_framework_os_renderer_wgpu_shell_pointer::CONTROL_ID;
use semio_framework_os_renderer_wgpu_shell_prefs::{ChromeGroupItem, DockTabNode, PrefsState, labels, retained_chrome_group_item_width};
use semio_framework_os_renderer_wgpu_shell_session::{SHELL_PALETTE_PLAN_ENTRY_CAPACITY, SessionCtx};
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_os_renderer_wgpu_shell_surfaces::{SettleState, ShellDetached, SurfaceState, TutorialState};
use semio_framework_os_renderer_wgpu_shell_sync::{Phase, SyncState};
use semio_framework_os_renderer_wgpu_shell_view_chrome::{ChromeViewCtx, ShellChromeCenteredBand, ShellChromeChildCursor, chrome_text_complete_step, document_opening_rect, document_opportunity_remains, note_dock_control_name, render_retained_chrome_group_item_step, render_retained_panel_chrome_item_step, shell_chrome_centered_band, shell_chrome_clip_horizontal};
use semio_framework_os_renderer_wgpu_shell_view_overlays::{OverlaysViewCtx, chrome_status_accessibility_node};
use semio_framework_os_renderer_wgpu_shell_view_paint::{PLUGIN_INSTALL_TOP_GAP, RetainedChromeGroupStep, SHELL_WINDOW_PAINT_OPPORTUNITIES, ShellChromeBandSpan, ShellUtilityPath, plugin_install_action_rect, scene_chrome_labels, surface_fits_overlay, transient_notice_tone};
use semio_framework_os_renderer_wgpu_shell_window_actions::{render_tutorial_gesture_overlay_node, window_actions_rect, window_search_rect};
use semio_framework_os_renderer_wgpu_shell_windows::{PanelAnchor, WindowState};
use semio_framework_os_renderer_wgpu_dock::{DockRenderContext, drop_zone_indicator_rect};
use semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::HubConnectionState;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::{UiDocumentFrameCursor, begin_ui_document_opportunity, render_ui_document_step};
use store_sync::PresencePeer;
use ui_contract::UiText;
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, HitKind, HitTarget, IconAtlas, InputState, Level, Rect, RetainedGlyphStep, RetainedTextFlow, Theme, TreeDropPosition, paint_retained_glyph_step_flowed, push_chrome_group_border};

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_view_panels__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_panels__theme_file_import_and_accessibility;
pub use staged_view_panels__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🎬️shell-actions/🦀️.rs"]
mod staged_view_panels__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/📤️icon-export/🦀️.rs"]
mod staged_view_panels__icon_export;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🔖️identity-and-types/🦀️.rs"]
mod staged_view_panels__identity_and_types;
pub use staged_view_panels__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🖱️pointer-input/🦀️.rs"]
mod staged_view_panels__pointer_input;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_panels__chrome_frame_and_main_window;
pub use staged_view_panels__chrome_frame_and_main_window::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_panels__chrome_overlays_and_tour;
pub use staged_view_panels__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_view_panels__navbar_footer_and_overlays;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🧰️window-actions-and-commands/🦀️.rs"]
mod staged_view_panels__window_actions_and_commands;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_panels__chrome_paint_and_navbar;
pub use staged_view_panels__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🧺️bundle/🦀️.rs"]
mod staged_view_panels__bundle;
pub use staged_view_panels__bundle::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
