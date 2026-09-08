use super::*;
use crate::{Puzzle2dEdge, Puzzle2dHandle, Puzzle2dNode, Puzzle2dNodeAnchor};

//#region 🧸️Fixtures
fn parent_child_snapshot() -> Puzzle2dSnapshot {
    // p (Fixed, off-origin) --e-- c (Derived): edge x/y offsets place c relative to p.
    let p = Puzzle2dNode { id: "p".into(), x: 5.0, y: 7.0, anchor: Puzzle2dNodeAnchor::Fixed, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
    let c = Puzzle2dNode { id: "c".into(), anchor: Puzzle2dNodeAnchor::Derived, handles: vec![Puzzle2dHandle { id: "h".into(), ..Default::default() }], ..Default::default() };
    let e = Puzzle2dEdge { id: "e".into(), source: "p:h".into(), target: "c:h".into(), x: 3.0, y: -2.0, ..Default::default() };
    Puzzle2dSnapshot { schema: crate::PUZZLE_2D_SCHEMA.to_string(), camera: Default::default(), nodes: vec![p, c], edges: vec![e], meta: Default::default() }
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
