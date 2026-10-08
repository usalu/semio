//! 🔺️ Sparse diff builder for `ReplacePartGrip` — patches the fields of one grip inside the owner part. An absent
//! part or an absent grip is `mutation.target-missing`; a replacement that keeps every field is the `mutation.no-op`
//! warning. A replacement keeps the addressed grip's id.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dGrip2dPatch, Puzzle5dGrip3dPatch, Puzzle5dGripPatch, Puzzle5dGripsDelta, Puzzle5dPartPatch, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplacePartGrip, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if payload.new_grip.id != payload.grip_id {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a replacement grip keeps the addressed grip's id", vec![payload.part_id.clone(), payload.grip_id.clone()]);
    }
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part-grip", payload.part_id), vec![payload.part_id.clone()]);
    };
    let Some(grip) = part.grips.iter().find(|grip| grip.id == payload.grip_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Grip \"{}\" not found on part \"{}\".", payload.grip_id, payload.part_id), vec![payload.grip_id.clone()]);
    };
    let next = &payload.new_grip;
    let patch = Puzzle5dGripPatch {
        grip_kind: (next.grip_kind != grip.grip_kind).then(|| next.grip_kind.clone()),
        grip_2d: Some(Puzzle5dGrip2dPatch {
            angle: (next.grip_2d.angle != grip.grip_2d.angle).then_some(next.grip_2d.angle),
            grip_kind: (next.grip_2d.grip_kind != grip.grip_2d.grip_kind).then(|| next.grip_2d.grip_kind.clone()),
            radius: (next.grip_2d.radius != grip.grip_2d.radius).then_some(next.grip_2d.radius),
        })
        .filter(|nested| !nested.is_empty()),
        grip_3d: Some(Puzzle5dGrip3dPatch {
            position: (next.grip_3d.position != grip.grip_3d.position).then_some(next.grip_3d.position),
            direction: (next.grip_3d.direction != grip.grip_3d.direction).then_some(next.grip_3d.direction),
            radius: (next.grip_3d.radius != grip.grip_3d.radius).then_some(next.grip_3d.radius),
            label: (next.grip_3d.label != grip.grip_3d.label).then(|| next.grip_3d.label.clone()),
        })
        .filter(|nested| !nested.is_empty()),
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.part_id.clone()])]);
    }
    let part_patch = Puzzle5dPartPatch { grips: Some(Puzzle5dGripsDelta::patching(payload.grip_id.clone(), patch)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle5dDiff { parts: Some(Puzzle5dPartsDelta::patching(payload.part_id.clone(), part_patch)), ..Default::default() })
}
//#endregion 🔖️Diff
