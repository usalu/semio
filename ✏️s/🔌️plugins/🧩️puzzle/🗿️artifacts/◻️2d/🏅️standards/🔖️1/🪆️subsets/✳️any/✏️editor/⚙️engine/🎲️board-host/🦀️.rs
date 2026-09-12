//! 🎲️ Puzzle 2d app engine — the themed board hosts: `BoardHost` constructors wired to this
//! plugin's metabolism icon table, plus the scene-sync, camera/LOD, hit-test, selection and
//! area-select laws they must satisfy.

use crate::editor::puzzle2d::engine::icons::puzzle_themed_icon_lookup;
use crate::editor::puzzle2d::engine::BoardHost;

/// 🎲️ A `BoardHost` for the directed port graph, painting icons from this plugin's metabolism table.
pub fn puzzle_board_host() -> BoardHost {
    let mut h = BoardHost::new();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

/// 🎲️ The undirected ("normal") variant of [`puzzle_board_host`].
pub fn puzzle_board_host_normal() -> BoardHost {
    let mut h = BoardHost::new_normal();
    h.icon_paint_cache.themed_icon_lookup = puzzle_themed_icon_lookup;
    h
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

//#endregion 🧪️Tests
