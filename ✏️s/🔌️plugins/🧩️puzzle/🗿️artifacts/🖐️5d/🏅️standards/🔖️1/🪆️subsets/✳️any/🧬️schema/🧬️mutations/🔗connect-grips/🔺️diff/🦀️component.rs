//! 🔺️ Sparse diff builder for `ConnectGrips` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dFastenersDelta};
use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectGrips, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.fasteners.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "already connected").at(vec![payload.id.clone()])]);
    }
    let fastener = Puzzle5dFastener {
        id: payload.id.clone(),
        source: payload.source.clone(),
        target: payload.target.clone(),
        fastener_kind: payload.fastener_kind.clone(),
        gap: payload.gap,
        shift: payload.shift,
        rise: payload.rise,
        rotation: payload.rotation,
        turn: payload.turn,
        tilt: payload.tilt,
        x: payload.x,
        y: payload.y,
    };
    protocol::MutationOutcome::new(Puzzle5dDiff { fasteners: Some(Puzzle5dFastenersDelta { added: vec![fastener], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
