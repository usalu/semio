//! 🖼️ Static metabolism icon catalogue for the Puzzle 2d editor: one module per icon family so every asset crate stays inside the source gate.

pub use semio_s_artifact_puzzle_2d_engine::editor::puzzle2d::engine::icons::board_metabolism_icons::tambour_whole;

pub use semio_s_artifact_puzzle_2d_tambour_end_storeys::editor::puzzle2d::engine::icons::board_metabolism_icons::tambour_end_storeys;

pub use semio_s_artifact_puzzle_2d_tambour_single_storey::editor::puzzle2d::engine::icons::board_metabolism_icons::tambour_single_storey;

#[path = "💊️capsules/🦀️.rs"]
pub mod capsules;

pub use semio_s_artifact_puzzle_2d_tambour_single_storey::editor::puzzle2d::engine::icons::board_metabolism_icons::ellipsoid_capsules;

/// 🧬️ Resolves a canonical metabolism icon identity to its bundled SVG source.
pub fn board_metabolism_icon_svg(key: &str) -> Option<&'static str> {
    let key = key.trim();
    tambour_whole::icon_svg(key)
        .or_else(|| tambour_end_storeys::icon_svg(key))
        .or_else(|| tambour_single_storey::icon_svg(key))
        .or_else(|| capsules::icon_svg(key))
        .or_else(|| ellipsoid_capsules::icon_svg(key))
}
