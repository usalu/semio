//! 🔺️ Sparse diff builder for `ConnectHandles` — a real append-only insert (never a
//! whole-snapshot capture). No-op when the id already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle2dDiff, Puzzle2dEdgesDelta};
use crate::{Puzzle2dEdge, Puzzle2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::puzzle2d_finite;

//#region 🔖️Diff
pub fn diff(payload: &super::ConnectHandles, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
    if let Err(reason) = puzzle2d_finite(&[("gap", payload.gap), ("shift", payload.shift), ("rise", payload.rise), ("rotation", payload.rotation), ("turn", payload.turn), ("tilt", payload.tilt), ("x", payload.x), ("y", payload.y)]) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![payload.id.clone()]);
    }
    if base.edges.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.id.clone()])]);
    }
    let edge = Puzzle2dEdge {
        id: payload.id.clone(),
        source: payload.source.clone(),
        target: payload.target.clone(),
        edge_kind: payload.edge_kind.clone(),
        gap: payload.gap,
        shift: payload.shift,
        rise: payload.rise,
        rotation: payload.rotation,
        turn: payload.turn,
        tilt: payload.tilt,
        x: payload.x,
        y: payload.y,
        source_tip: payload.source_tip.clone(),
        target_tip: payload.target_tip.clone(),
        visible: None,
        locked: None,
    };
    protocol::MutationOutcome::new(Puzzle2dDiff { edges: Some(Puzzle2dEdgesDelta { added: vec![edge], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
