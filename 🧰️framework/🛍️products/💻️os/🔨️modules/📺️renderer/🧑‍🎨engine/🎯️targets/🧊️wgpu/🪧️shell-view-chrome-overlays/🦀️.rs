//! 🪧️ Shell chrome overlay steps: tutorial bar, context menu, tooltip, agent chat header, notices and status stacks.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪧️view-chrome-overlays`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate. Split from `shell-view-chrome`, which this crate builds on.

use semio_renderer_wgpu_shell_view_chrome::*;
use semio_renderer_wgpu_shell_async::{chrome_now_ms, directory_now_ms};
use semio_renderer_wgpu_shell_chrome_state::{ChromeState, ChromeTooltipHover, chrome_tooltip_ready};
use semio_renderer_wgpu_shell_commands::CommandState;
use semio_renderer_wgpu_shell_directory::DirectoryState;
use semio_renderer_wgpu_shell_doc_open::{OpeningState, ShellDocumentOpenPhase};
use semio_renderer_wgpu_shell_dock::ShellDockState;
use semio_renderer_wgpu_shell_docs::DocumentState;
use semio_renderer_wgpu_shell_history::HistoryState;
use semio_renderer_wgpu_shell_input::{ShellInputState, note_chrome_control_name, with_chrome_control_names};
use semio_renderer_wgpu_shell_plugins::{PluginState, ShellPluginInstallPhase};
use semio_renderer_wgpu_shell_prefs::{ChromeGroupItem, PrefsState, retained_chrome_group_item_width};
use semio_renderer_wgpu_shell_state::ShellState;
use semio_renderer_wgpu_shell_surfaces::{SettleState, SurfaceState, TutorialMode, TutorialState, tutorial_bar_height};
use semio_renderer_wgpu_shell_sync::SyncState;
use semio_renderer_wgpu_shell_view_paint::{tutorial_chapter_tick_hit_rect, tutorial_chapter_tick_rect, DOCUMENT_OPEN_CANCEL_CONTROL_ID, PLUGIN_INSTALL_CANCEL_CONTROL_ID, RetainedChromeGroupStep, TRANSIENT_NOTICE_CLOSE_CONTROL_ID, chrome_icon, plugin_install_action_rect, plugin_install_banner_text, transient_notice_close_rect, transient_notice_tone, tutorial_scrub_progress, tutorial_scrub_target_ms};
use semio_renderer_wgpu_shell_window_actions::WindowActionsCtx;
use semio_renderer_wgpu_shell_windows::{PanelAnchor, WindowState};
use semio_renderer_wgpu_shell_vocab::shell_chrome_string;
use ui_contract::UiText;
use semio_ui_declaration::layout::ActionDescriptor;
use semio_ui_wgpu_input::{HitKind, HitTarget, InputState};
use semio_ui_draw_contract::{DrawList, IconAtlas};
use semio_ui_text::FontAtlas;
use semio_ui_theme::{Level, Theme};
use semio_ui_theme::geometry::Rect;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪧️view-chrome-overlays/🧭️overlay-steps/🦀️.rs"]
mod staged_view_chrome_overlays__overlay_steps;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪧️view-chrome-overlays/🧺️bundle/🦀️.rs"]
mod staged_view_chrome_overlays__bundle;
pub use staged_view_chrome_overlays__bundle::*;
