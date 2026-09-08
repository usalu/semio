//! 🖌️ Puzzle 2d app engine — the brush laws: slot preview/commit/cancel, candidate ordering by
//! handle proximity, per-node-kind compatibility enumeration, and the deterministic fill session.

//#region 🧪️Tests
#[cfg(test)]
#[allow(
    clippy::approx_constant,
    reason = "3.14159 is verbatim fixture data (a handle angle in a scene JSON literal), carried over unchanged from the pre-consolidation engine crate; swapping in std::f64::consts::PI would alter the recorded test input."
)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
