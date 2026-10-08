//! 🔺️ Sparse diff builder for `ConnectVortices` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::{Puzzle3dAttraction, Puzzle3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectVortices, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.attractions.iter().any(|entry| entry.id == payload.id) {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "already connected").at(vec![payload.id.clone()])]);
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
    let reordered = payload.index.filter(|index| *index < base.attractions.len()).map(|index| {
        let mut order: Vec<String> = base.attractions.iter().map(|entry| entry.id.clone()).collect();
        order.insert(index, payload.id.clone());
        order
    });
    protocol::MutationOutcome::new(Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta::adding(attraction, reordered)), ..Default::default() })
}
//#endregion 🔖️Diff
