//! 🎛 `flat-position` — one named inference: each node's resolved `(x, y)` position after the
//! compose-parity fastened layout — `Fixed`-anchor nodes keep their stored coordinates, `Derived`-
//! anchor nodes get theirs BFS-walked from their connecting edge's params (gap/shift/rise/rotation/
//! turn/tilt/x/y), the same graph-position concept `🧊️3d`'s own `🎛flat-position/` and
//! `🔱️trinity/🔌️jack`'s own `🎛flat-position/` carry for their artifacts. Reuses the inference
//! family root's own `fastened_layout_snapshot`'s existing compose-parity math on a snapshot clone
//! rather than re-deriving it here (this repo's own "if code is repeated, it must be close to each
//! other" rule taken to its natural conclusion: don't repeat it at all when the source of truth
//! already exists) — a plain whole-snapshot BFS pass, so, per the family root's own rationale
//! (mirroring jack's `🧭topology`/`🎛flat-position` and puzzle3d's own sibling), no
//! `InferredField`/incremental caching is needed here either.

use super::super::fastened_layout_snapshot;
use crate::Puzzle2dSnapshot;
use std::collections::BTreeMap;

//#region 🔖️FlatPosition
/// 🎛 One node's resolved position.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dFlatPositionXy {
    pub x: f64,
    pub y: f64,
}

/// 🎛 Resolved `(x, y)` position per node id — covers every node, `Fixed` and `Derived` alike.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dFlatPosition {
    pub positions: BTreeMap<String, Puzzle2dFlatPositionXy>,
}

/// 📐️ Computes `flat-position` by running the existing `fastened_layout_snapshot` compose-parity
/// BFS on a snapshot clone and reading back every node's resolved `(x, y)` — deterministic because
/// `fastened_layout_snapshot` itself walks `nodes`/`edges` in fixture order with no randomness.
pub fn compute_flat_position(snapshot: &Puzzle2dSnapshot) -> Puzzle2dFlatPosition {
    let mut resolved = snapshot.clone();
    fastened_layout_snapshot(&mut resolved);
    let positions = resolved.nodes.iter().map(|node| (node.id.clone(), Puzzle2dFlatPositionXy { x: node.x, y: node.y })).collect();
    Puzzle2dFlatPosition { positions }
}
//#endregion 🔖️FlatPosition

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
