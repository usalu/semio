//! 🎛 `flat-position` — one named inference: each node's resolved `(x, y)` position after the
//! compose-parity fastened layout — `Fixed`-anchor nodes keep their stored coordinates, `Derived`-
//! anchor nodes get theirs BFS-walked from their connecting edge's params (gap/shift/rise/rotation/
//! turn/tilt/x/y), the same graph-position concept `🧊️3d`'s own `🎛flat-position/` and
//! `🔱️trinity/🔌️jack`'s own `🎛flat-position/` carry for their artifacts. Reuses
//! `⚙️engine/📐️layout::fastened_layout_snapshot`'s existing compose-parity math on a snapshot clone
//! rather than re-deriving it here (this repo's own "if code is repeated, it must be close to each
//! other" rule taken to its natural conclusion: don't repeat it at all when the source of truth
//! already exists) — a plain whole-snapshot BFS pass, so, per the family root's own rationale
//! (mirroring jack's `🧭topology`/`🎛flat-position` and puzzle3d's own sibling), no
//! `InferredField`/incremental caching is needed here either.

use crate::artifacts::puzzle2d::standards::v1::engine::layout::fastened_layout_snapshot;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️FlatPosition
/// 🎛 One node's resolved position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle2dFlatPositionXy {
    pub x: f64,
    pub y: f64,
}

/// 🎛 Resolved `(x, y)` position per node id — covers every node, `Fixed` and `Derived` alike.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
mod tests {
    use super::*;
    use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};

    //#region 🧸️Fixtures
    fn parent_child_snapshot() -> Puzzle2dSnapshot {
        // p (Fixed, off-origin) --e-- c (Derived): edge x/y offsets place c relative to p.
        let p = Puzzle2dNode {
            id: "p".into(),
            x: 5.0,
            y: 7.0,
            anchor: Puzzle2dNodeAnchor::Fixed,
            handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }],
            ..Default::default()
        };
        let c = Puzzle2dNode {
            id: "c".into(),
            anchor: Puzzle2dNodeAnchor::Derived,
            handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }],
            ..Default::default()
        };
        let e = Puzzle2dEdge { id: "e".into(), source: "p:h".into(), target: "c:h".into(), x: 3.0, y: -2.0, ..Default::default() };
        Puzzle2dSnapshot { schema: crate::artifacts::puzzle2d::PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: vec![p, c], edges: vec![e], meta: Default::default() }
    }
    //#endregion 🧸️Fixtures

    //#region 🧪️FlatPositionLaws
    #[test]
    fn inference_determinism_law() {
        let snapshot = parent_child_snapshot();
        assert_eq!(compute_flat_position(&snapshot), compute_flat_position(&snapshot));
    }

    #[test]
    fn inference_default_law() {
        assert_eq!(compute_flat_position(&Puzzle2dSnapshot::default()), Puzzle2dFlatPosition::default());
    }

    #[test]
    fn fixed_parent_keeps_its_coordinates_and_derived_child_offsets_by_edge_xy() {
        let flat = compute_flat_position(&parent_child_snapshot());
        let p = flat.positions.get("p").expect("p present");
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 7.0);
        let c = flat.positions.get("c").expect("c present");
        // DIAGRAM_HORIZONTAL_SCALE = 3.0633 (⚙️engine/📐️geometry/🎛flatten's real diagram constant,
        // reused verbatim by fastened_layout_snapshot's off-origin-parent branch).
        assert!((c.x - (5.0 + 3.0 * 3.0633)).abs() < 1e-9, "c.x = {}", c.x);
        assert!((c.y - (7.0 + -2.0 * 3.0633)).abs() < 1e-9, "c.y = {}", c.y);
    }
    //#endregion 🧪️FlatPositionLaws
}
//#endregion 🧪️Tests
