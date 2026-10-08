//! 🔺️ Sparse diff builder for `ReplaceObjectVortex` — patches the presentation fields of one vortex inside
//! the owner object. An absent object or an absent vortex is `mutation.target-missing`; a replacement that
//! keeps every field is the `mutation.no-op` warning. A replacement keeps the addressed vortex's id.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectsDelta, Puzzle3dVortexPatch, Puzzle3dVorticesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReplaceObjectVortex, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if payload.new_vortex.id != payload.vortex_id {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a replacement vortex keeps the addressed vortex's id", vec![payload.object_id.clone(), payload.vortex_id.clone()]);
    }
    let Some(object) = base.objects.iter().find(|entry| entry.id == payload.object_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object-vortex", payload.object_id), vec![payload.object_id.clone()]);
    };
    let Some(vortex) = object.vortices.iter().find(|vortex| vortex.id == payload.vortex_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vortex \"{}\" not found on object \"{}\".", payload.vortex_id, payload.object_id), vec![payload.vortex_id.clone()]);
    };
    let next = &payload.new_vortex;
    let patch = Puzzle3dVortexPatch {
        vortex_kind: (next.vortex_kind != vortex.vortex_kind).then(|| next.vortex_kind.clone()),
        label: (next.label != vortex.label).then(|| next.label.clone()),
        position: (next.position != vortex.position).then_some(next.position),
        direction: (next.direction != vortex.direction).then_some(next.direction),
        radius: (next.radius != vortex.radius).then_some(next.radius),
        hidden: (next.hidden != vortex.hidden).then_some(next.hidden),
        locked: (next.locked != vortex.locked).then_some(next.locked),
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.object_id.clone()])]);
    }
    let object_patch = Puzzle3dObjectPatch { vortices: Some(Puzzle3dVorticesDelta::patching(payload.vortex_id.clone(), patch)), ..Default::default() };
    protocol::MutationOutcome::new(Puzzle3dDiff { objects: Some(Puzzle3dObjectsDelta::patching(payload.object_id.clone(), object_patch)), ..Default::default() })
}
//#endregion 🔖️Diff
