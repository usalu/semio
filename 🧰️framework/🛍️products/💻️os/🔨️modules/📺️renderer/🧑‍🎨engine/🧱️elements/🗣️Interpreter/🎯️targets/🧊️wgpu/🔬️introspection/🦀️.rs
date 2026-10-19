//! 🔬️ Introspection dumps and their wasm exports.
//!
//! The structure, frame, mesh and chrome dump types and builders, the accessibility dump and the wasm exports.

#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use crate::chrome_ledger::project_chrome_dump;
#[cfg(target_arch = "wasm32")]
use crate::chrome_ledger::CHROME_LEDGER;
#[cfg(any(test, feature = "testkit"))]
use crate::chrome_ledger::{accessibility_window_is_visible, begin_accessibility_visible_documents, chrome_action_args, chrome_hit_row, ledger_publish_hits, ledger_publish_surfaces, ledger_push_action, note_accessibility_visible_document, note_chrome_accessibility, publish_accessibility_visible_documents, world_camera_ledger_row, ChromeLedger, CHROME_ACTION_ARGS_BYTES, CHROME_ACTION_CAPACITY, WORLD_CAMERA_LEDGER};
use crate::chrome_ledger::{ACCESSIBILITY_VISIBLE_WINDOWS, CHROME_ACCESSIBILITY, SHELL_CHROME_ACCESSIBILITY_WINDOW_ID};
use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::focused_vfs_control;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use crate::mesh_and_board2d_stats::build_board2d_dump;
#[cfg(target_arch = "wasm32")]
use crate::mesh_and_board2d_stats::build_mesh_stats;
#[cfg(any(test, feature = "testkit"))]
use crate::mesh_and_board2d_stats::{board2d_surface, mesh_stats_for_scene, walk_board2d, walk_mesh_stats};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_renderer_wgpu_contracts::UI_ENGINE;
use semio_framework_os_renderer_wgpu_ui_engine_focus::text_and_table_focus::{focused_table_editable_text_address, focused_table_editable_text_draft, focused_text_editor, FOCUSED_TABLE_STEPPER};
#[cfg(any(test, feature = "testkit"))]
use crate::element_contracts::fixture_scene_pointer_target;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))]
use serde_json::Value;
#[cfg(any(target_arch = "wasm32", any(test, feature = "testkit")))] use semio_framework_ui_declaration::ui::UiState;
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_declaration::layout::ActionDescriptor;
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_declaration::ui::UiPresence;
use semio_framework_ui_arena::NodeId;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::Theme;
use semio_framework_ui_declaration::ui::UiNode;

//#region 🔬️Introspection
/** 🔬️ Structural + frame-stats dump for the wgpu↔React UI-parity headless test harness (see
 * `.🧬semio/🦑️repo/🎫️tickets/26/07/11/WGPU-RENDERER-FULL-PARITY`): walks the SAME `UI_ENGINE` retained façade
 * `render_ui_node` (above) already drives, so every dump reflects exactly what was last laid
 * out/painted — never a second, independent measurement pass. Deliberately scoped to ONE window's
 * content tree: shell chrome/navbar/footer/dock are rendered by this crate's own immediate-mode
 * widgets code directly into the composited canvas frame, never through `UI_ENGINE` at all, so
 * they're structurally unreachable from here — out of scope by construction, not by filtering.
 * Exported to JS the same way `semioWgpuMount`/`uploadIconAtlas` already are (bare
 * `#[wasm_bindgen(js_name = "...")]` free functions) — no new loading path invented for these two;
 * see the `🔬️IntrospectionExports` sub-region below for exactly how they end up reachable. */

#[path = "🧾️dump-types-paths-and-walk/🦀️.rs"]
mod dump_types_paths_and_walk;
#[cfg(target_arch="wasm32")]
use dump_types_paths_and_walk::{DumpStructure, DumpViewport};
use dump_types_paths_and_walk::{DumpAccessibility, DumpAccessibilityWindow};
#[cfg(any(target_arch="wasm32",any(test,feature="testkit")))]
pub use dump_types_paths_and_walk::{
    ui_node_kind_tag, ui_node_path_segment, walk_dump, DumpFrameStats, DumpMesh, DumpMeshInstance,
    DumpMeshStats, DumpMeshSurface,
};
#[cfg(any(target_arch="wasm32",any(test,feature="testkit")))]
use dump_types_paths_and_walk::dump_window_id;
pub use dump_types_paths_and_walk::dump_window_ids;

#[path = "🏗️structure-dump-and-table-accessibility/🦀️.rs"]
mod structure_dump_and_table_accessibility;
#[cfg(target_arch="wasm32")]
use structure_dump_and_table_accessibility::build_structure_dump;
#[cfg(any(target_arch="wasm32",any(test,feature="testkit")))]
use structure_dump_and_table_accessibility::{is_glyph_instance, layer_is_nonempty};
pub use structure_dump_and_table_accessibility::{
    scene_virtual_accessibility_node_id, table_button_accessibility_entries,
    table_editable_text_accessibility_entries, table_stepper_accessibility_entries,
};
use structure_dump_and_table_accessibility::{
    append_table_button_accessibility_nodes, append_table_editable_text_accessibility_nodes,
    append_table_stepper_accessibility_nodes,
};

#[path = "♿️vfs-feed-world-and-timeline-accessibility/🦀️.rs"]
mod vfs_feed_world_and_timeline_accessibility;
pub use vfs_feed_world_and_timeline_accessibility::{
    block_list_accessibility_entries, event_feed_accessibility_entries,
    graph_timeline_accessibility_entries, vfs_accessibility_entries,
};
use vfs_feed_world_and_timeline_accessibility::{
    append_block_list_accessibility_nodes, append_event_feed_accessibility_nodes,
    append_graph_timeline_accessibility_nodes, append_vfs_accessibility_nodes,
    append_world3d_accessibility_nodes,
};

#[path = "📤️editor-accessibility-frame-stats-and-exports/🦀️.rs"]
mod editor_accessibility_frame_stats_and_exports;
pub use editor_accessibility_frame_stats_and_exports::{
    build_accessibility_dump, text_editor_accessibility_entries,
};
#[cfg(any(target_arch="wasm32",any(test,feature="testkit")))]
pub use editor_accessibility_frame_stats_and_exports::build_frame_stats;
#[cfg(target_arch="wasm32")]
pub use editor_accessibility_frame_stats_and_exports::{
    dump_accessibility, dump_board2d, dump_chrome, dump_frame_stats, dump_mesh_stats,
    dump_structure,
};

//#region 🔬️IntrospectionExports
/// 📤️ `dumpStructure()`/`dumpFrameStats()` — reachable exactly like `semioWgpuMount`/
/// `uploadIconAtlas` already are: Trunk's dev-server boot glue (`framework/renderer/wgpu/js/
/// 🟦️.ts`) waits for `window.wasmBindings` then calls exports straight off it, so these land at
/// `window.wasmBindings.dumpStructure()`/`window.wasmBindings.dumpFrameStats()` there; the library
/// boot path (`framework/renderer/wgpu/index.ts`'s `bootFrameworkOsWgpu`) instead calls them on its
/// own dynamically-imported module object. No new loading path invented for either.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

//#endregion 🔬️IntrospectionExports

//#endregion 🔬️Introspection
