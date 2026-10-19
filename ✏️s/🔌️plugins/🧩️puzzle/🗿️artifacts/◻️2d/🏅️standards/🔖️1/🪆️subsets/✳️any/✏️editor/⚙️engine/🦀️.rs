//! 📱️ app-side (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1e: an artifact is a `🧬️schema`
//! ⚙️ Puzzle 2d app engine — headless compute + interactive host over the board scene, rehomed
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
//! 🌉️ Externally reachable at `puzzle::editor::puzzle2d::engine::*`. The framework OS renderer never links
//! this module: its wgpu `Board2d` surface holds a bare `BoardHost` and receives this plugin's brush naming as
//! scene data (`Board2dScene::placement_naming`, published from `board_host::PUZZLE_2D_BOARD_NAMING`). The
//! module and its `board_host` child stay `pub` for the plugin's own wasm session and the compositions that
//! re-export it.
//!
//! 🧭️ Placement rule for helpers reaching across nodes: a helper with exactly ONE consumer lives in
//! that consumer's file; two or more consumers put it here.

pub use semio_framework_canvas as canvas;
pub use canvas::CubicBez;
pub use canvas::Point;
pub use canvas::Vec2;
pub use semio_framework_os_infinite::*;

//#region 🔖️Puzzle2dExtension
/// 🧩️ Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl semio_framework_canvas::CanvasExtension for Puzzle2dExtension {
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
