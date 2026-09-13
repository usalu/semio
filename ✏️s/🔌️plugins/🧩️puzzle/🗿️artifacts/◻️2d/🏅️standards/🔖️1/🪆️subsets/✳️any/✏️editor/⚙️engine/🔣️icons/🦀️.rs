//! 🔣️ Puzzle 2d app engine — the metabolism icon table: this module reads the shared
//! `🌱️metabolism/🔣️icons` asset directory through a static `board_metabolism_icon_svg(key)` match table
//! per SVG, keyed by the bare stem after the repo's emoji filename prefix. The board's icon codec
//! reaches it through [`puzzle_themed_icon_lookup`].

#[path = "🌱️metabolism/🦀️.rs"]
mod board_metabolism_icons;

/// 🔣️ Resolves a board catalog icon key to its SVG source, or `None` when the key is not a
/// metabolism asset (the codec then falls back to typst-math / emoji rendering).
pub fn puzzle_themed_icon_lookup(key: &str) -> Option<&'static str> {
    board_metabolism_icons::board_metabolism_icon_svg(key)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
