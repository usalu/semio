//! 🔎 Test host of the laws in `🔬️wgpu-introspection`: they name the monolith (scenes, engine canvas, the frame fixtures), so they run in the renderer test unit with the namespace of the `introspection` module of the `ui-engine` partition around them.

#[cfg(test)]
use crate::element_contracts::fixture_scene_pointer_target;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::accessibility_window_is_visible;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::begin_accessibility_visible_documents;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::chrome_action_args;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::chrome_hit_row;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::ledger_publish_hits;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::ledger_publish_surfaces;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::ledger_push_action;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_accessibility_visible_document;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::note_chrome_accessibility;
#[cfg(any(target_arch = "wasm32", test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::project_chrome_dump;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::publish_accessibility_visible_documents;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::world_camera_ledger_row;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::ChromeLedger;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::CHROME_ACTION_ARGS_BYTES;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::CHROME_ACTION_CAPACITY;
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::SHELL_CHROME_ACCESSIBILITY_WINDOW_ID;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::WORLD_CAMERA_LEDGER;
pub(crate) use semio_framework_os_renderer_wgpu_ui_engine_dump::introspection::*;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::board2d_surface;
#[cfg(any(target_arch = "wasm32", test))]
use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::build_board2d_dump;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::mesh_stats_for_scene;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::walk_board2d;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_dump::mesh_and_board2d_stats::walk_mesh_stats;
use semio_framework_ui_declaration::ui::UiNode;
#[cfg(test)]
use semio_framework_ui_layout::layout::ActionDescriptor;
#[cfg(test)]
use semio_framework_ui_layout::layout::UiPresence;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
#[cfg(any(target_arch = "wasm32", test))]
use serde_json::Value;

#[path = "../../../../../../../🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs"]
mod tests;
