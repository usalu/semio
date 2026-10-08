//! 🔺️ Sparse diff builder for `ReplaceAttractionGeometry` — patches the one addressed attraction in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dAttractionPatch, Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceAttractionGeometry, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.attractions.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attraction", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dAttractionPatch {
        gap: (payload.new_gap != item.gap).then_some(payload.new_gap),
        shift: (payload.new_shift != item.shift).then_some(payload.new_shift),
        rise: (payload.new_rise != item.rise).then_some(payload.new_rise),
        rotation: (payload.new_rotation != item.rotation).then_some(payload.new_rotation),
        turn: (payload.new_turn != item.turn).then_some(payload.new_turn),
        tilt: (payload.new_tilt != item.tilt).then_some(payload.new_tilt),
        x: (payload.new_x != item.x).then_some(payload.new_x),
        y: (payload.new_y != item.y).then_some(payload.new_y),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        attractions: Some(Puzzle3dAttractionsDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
