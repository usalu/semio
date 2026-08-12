//! 🔺️ Sparse diff builder for `ConnectVortices` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::{Puzzle3dAttraction, Puzzle3dSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ConnectVortices, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
    if base.attractions.iter().any(|entry| entry.id == payload.id) {
        return Puzzle3dDiff::default();
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
    Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta { added: vec![attraction], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
