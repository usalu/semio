//! 🔺️ Sparse diff builder for `ConnectVortices` — a real append-only insert. No-op when the id
//! already exists in `base`.
use semio_s_artifact_puzzle_3d_diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use {semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction, semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot};

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
    let index = payload.index.map_or(base.attractions.len(), |index| index.min(base.attractions.len()));
    protocol::MutationOutcome::new(Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta::insertion(index, attraction)), ..Default::default() })
}
//#endregion 🔖️Diff
