//! 🔺️ Diff for `ResizeVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Block3dVorticesDelta, Block3dVorticesPatchEntry, Block3dVortexTemplatePatch};

//#region 🔖️Diff
pub fn diff(payload: &super::ResizeVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.vortices.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    };
    if existing.radius == payload.new_radius {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    let patch = Block3dVortexTemplatePatch { radius: Some(payload.new_radius), ..Default::default() };
    protocol::MutationOutcome::new(Block3dDiff { vortices: Some(Block3dVorticesDelta { patched: vec![Block3dVorticesPatchEntry { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
