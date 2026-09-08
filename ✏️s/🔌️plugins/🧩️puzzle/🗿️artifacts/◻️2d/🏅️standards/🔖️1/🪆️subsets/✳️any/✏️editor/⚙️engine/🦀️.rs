//! ⚙️ Puzzle 2d app engine — headless compute + interactive host over the board scene, rehomed
//! app-side (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e: an artifact is a `🧬️schema`
//! plus a `🚪️io` system, never an engine — the puzzle2d artifact's old `⚙️engine` owned a genuinely
//! stateful/interactive `BoardHost`, so it moves to the app that edits the artifact). Thin domain
//! facade over the `infinite-board` port-directed graph kernel: it re-exports that kernel's whole
//! surface (`BoardEngine`, `BoardHost`, the vector `canvas`, the force/hierarchical layouts) under
//! one name, tags it with the `puzzle.2d` canvas extension.
//!
//! 📚️ Sibling topic files: `🎲️board-host/🦀️.rs` (the themed host constructors + their
//! scene/selection/hit-test laws), `🔗️linking/🦀️.rs` (handle-to-handle wiring and
//! compatibility laws), `🖌️brush/🦀️.rs` (brush slot/fill session laws),
//! `📐️layout/🦀️.rs` (the redraw layout dispatcher), `🔣️icons/🦀️.rs` (the
//! build-script-generated metabolism icon table and the SVG icon codec laws).
//!
//! 🌉️ Externally reachable at `puzzle::editor::puzzle2d::engine::*` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET moved this module here from the retired
//! `puzzle::apps::puzzle2d::engine::*`; the framework OS renderer's own call sites were the one real
//! cross-crate compile dependency on the old path and were repointed at this module in the same
//! ticket's W2-FIX lane) — the framework OS renderer
//! (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`)
//! holds a `BoardHost` and calls `board_host::puzzle_board_host()` directly, so this module and its
//! `board_host` child must both stay `pub`.
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here.

pub use canvas::{CubicBez, Point, Vec2};
pub use semio_framework_graph::canvas;
pub use semio_framework_graph::{
    apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value, apply_normal_undirected_redraw_layout_to_fixture_v1_json,
    apply_redraw_layout_to_fixture_v1_json as apply_ported_redraw_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_json, apply_undirected_force_graph_layout_to_fixture_v1_value, GraphExtension,
};
pub use semio_framework_os_infinite::{self as graph, *};

//#region 🔖️Puzzle2dExtension
/// 🧩️ Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl CanvasExtension for Puzzle2dExtension {
    fn extension_id(&self) -> &str {
        "puzzle.2d"
    }
}

impl GraphExtension for Puzzle2dExtension {}
//#endregion 🔖️Puzzle2dExtension

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
