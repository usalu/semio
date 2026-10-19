//! 🫧️ SurfaceState, TutorialState and SettleState components of the shell.
//!
//! 🧭️ Native and wasm32: the staging modules of the Shell element tree (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces`) are mounted here exactly as the `shell` parent mounted them; the `shell` module re-exports this crate.

extern crate infinite_canvas as infinite_world;
use semio_framework_os_renderer_wgpu_shell_chrome_state::ChromeState;
use infinite_world::world::{World3dState, WorldInteractionIntent, WorldInteractionPhase, enqueue_world3d_events, step_world3d_dynamic_retirement, world3d_catalogue_drop_origin, world3d_clear_catalogue_drop_preview, world3d_dynamic_retirement_terminal_is_empty, world3d_update_catalogue_drop_preview};
use semio_framework::kernel::UiDirtyScope;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_async::{Lane, WorkerPool};
use semio_framework_os_renderer_wgpu_contracts::AdmittedSurfaceMap;
use semio_framework_os_renderer_wgpu_scene_feeds::{NodeGraphSurface, TiledMapSurface};
use semio_framework_os_renderer_wgpu_scene_text_vfs::Board2dSurface;
use semio_framework_value::DslValue;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, VecDeque};
use ui_contract::UiDocumentLease;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::PointerModifiers;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🧬️component/🦀️.rs"]
mod staged_surfaces__component;
pub use staged_surfaces__component::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/⌨️keyboard-and-palette/🦀️.rs"]
mod staged_surfaces__keyboard_and_palette;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/⚙️settings-and-dock-persistence/🦀️.rs"]
mod staged_surfaces__settings_and_dock_persistence;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🎓️tutorial-runtime/🦀️.rs"]
mod staged_surfaces__tutorial_runtime;
pub use staged_surfaces__tutorial_runtime::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🎨️theme-file-import-and-accessibility/🦀️.rs"]
mod staged_surfaces__theme_file_import_and_accessibility;
pub use staged_surfaces__theme_file_import_and_accessibility::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🎬️shell-actions/🦀️.rs"]
mod staged_surfaces__shell_actions;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🏛️space-admin-and-settle/🦀️.rs"]
mod staged_surfaces__space_admin_and_settle;
pub use staged_surfaces__space_admin_and_settle::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🖱️pointer-input/🦀️.rs"]
mod staged_surfaces__pointer_input;
pub use staged_surfaces__pointer_input::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🖼️chrome-frame-and-main-window/🦀️.rs"]
mod staged_surfaces__chrome_frame_and_main_window;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🗨️chrome-overlays-and-tour/🦀️.rs"]
mod staged_surfaces__chrome_overlays_and_tour;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🧩️session-and-topology/🦀️.rs"]
mod staged_surfaces__session_and_topology;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🧭️navbar-footer-and-overlays/🦀️.rs"]
mod staged_surfaces__navbar_footer_and_overlays;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🧱️chrome-paint-and-navbar/🦀️.rs"]
mod staged_surfaces__chrome_paint_and_navbar;
pub use staged_surfaces__chrome_paint_and_navbar::*;
#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🫧️surfaces/🧵️shell-async-runners/🦀️.rs"]
mod staged_surfaces__shell_async_runners;
pub use staged_surfaces__shell_async_runners::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
