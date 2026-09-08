//! 🔗️ Puzzle 2d app engine — the handle-to-handle wiring laws: link-drag snapping, proximity
//! connect, the indirect ring pick, kind-compatibility filtering and the hidden/locked guards.
//! Behaviour lives in the `infinite-board` kernel; this node pins the puzzle-2d contract on it.

//#region 🧪️Tests
#[cfg(test)]
#[allow(
    clippy::approx_constant,
    reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input."
)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
