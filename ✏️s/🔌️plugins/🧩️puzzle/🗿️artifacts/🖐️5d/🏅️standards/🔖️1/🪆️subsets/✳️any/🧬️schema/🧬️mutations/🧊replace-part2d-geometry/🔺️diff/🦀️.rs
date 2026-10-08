//! 🔺️ Sparse diff builder for `ReplacePart2dGeometry` — patches the one addressed part in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dPart2dPatch, Puzzle5dPartPatch, Puzzle5dPartsDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplacePart2dGeometry, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle5dPartPatch {
        part_2d: Some(Puzzle5dPart2dPatch { shape: (payload.new_shape != item.part_2d.shape).then(|| payload.new_shape.clone()), radius: (payload.new_radius != item.part_2d.radius).then_some(payload.new_radius), width: (payload.new_width != item.part_2d.width).then_some(payload.new_width), height: (payload.new_height != item.part_2d.height).then_some(payload.new_height), ..Default::default() }).filter(|nested| !nested.is_empty()),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
