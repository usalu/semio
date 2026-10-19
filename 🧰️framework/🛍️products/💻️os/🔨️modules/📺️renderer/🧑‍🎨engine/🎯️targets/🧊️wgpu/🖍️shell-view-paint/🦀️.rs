//! 🖍️ Shell view paint helpers: chrome control presentation, accessibility roles and chrome groups.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

use semio_framework_os_renderer_wgpu_shell_plugins::{ShellPluginInstall, ShellPluginInstallPhase};
use semio_framework_os_renderer_wgpu_shell_pointer::floating_panel_max_width;
use semio_framework_os_renderer_wgpu_shell_surfaces::World3dComputeStatus;
use semio_framework_os_renderer_wgpu_shell_windows::PanelAnchor;
use semio_framework_os_kernel::os_directory::identity::Identity;
use semio_framework_os_renderer_wgpu_scene_canvas2d::ui_image_loading::framework_widget_context;
use semio_framework_os_renderer_wgpu_scene_state::{BlockListChromeLabels, SceneChromeLabels, VirtualFileSystemChromeLabels};
use semio_framework_os_renderer_wgpu_shell_vocab::shell_chrome_string;
use std::collections::HashMap;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::InputState;
use semio_framework_ui_theme::{Level, Rgba, Theme};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_wgpu_paint::{RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow, paint_retained_glyph_step_flowed};
use semio_framework_ui_declaration::utilities::UtilityNode;
use semio_framework_ui_wgpu_widget_kit::draw_text;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/⚓️dock-model-and-measures/🦀️.rs"]
mod staged_view_paint__dock_model_and_measures;
pub use staged_view_paint__dock_model_and_measures::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🎓️tutorial-runtime/🦀️.rs"]
mod staged_view_paint__tutorial_runtime;
pub use staged_view_paint__tutorial_runtime::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_view_paint__theme_file_import_and_accessibility;
pub use staged_view_paint__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_view_paint__space_admin_and_settle;
pub use staged_view_paint__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🔑️storage-prefs-and-i18n/🦀️.rs"]
mod staged_view_paint__storage_prefs_and_i18n;
pub use staged_view_paint__storage_prefs_and_i18n::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🔖️identity-and-types/🦀️.rs"]
mod staged_view_paint__identity_and_types;
pub use staged_view_paint__identity_and_types::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_view_paint__chrome_frame_and_main_window;
pub use staged_view_paint__chrome_frame_and_main_window::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_view_paint__chrome_overlays_and_tour;
pub use staged_view_paint__chrome_overlays_and_tour::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖍️view-paint/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_view_paint__chrome_paint_and_navbar;
pub use staged_view_paint__chrome_paint_and_navbar::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
