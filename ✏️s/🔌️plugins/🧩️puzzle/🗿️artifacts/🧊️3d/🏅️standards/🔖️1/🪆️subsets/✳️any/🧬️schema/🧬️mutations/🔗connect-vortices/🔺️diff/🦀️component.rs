//! 🔺️ Sparse diff builder for `ConnectVortices` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ConnectVortices, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.attractions.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "already connected").at(vec![payload.id.clone()])]);
    }
    let attraction = Puzzle3dAttraction {
        id: payload.id.clone(),
        attracting: payload.attracting.clone(),
        attracted: payload.attracted.clone(),
        gap: payload.gap,
        shift: payload.shift,
        rise: payload.rise,
        rotation: payload.rotation,
        turn: payload.turn,
        tilt: payload.tilt,
        x: payload.x,
        y: payload.y,
    };
    protocol::MutationOutcome::new(Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta { added: vec![attraction], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
