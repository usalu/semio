//! 🔺️ Sparse diff builder for `RemoveSpaceListMember` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ListEdit, ModelPatch, Rows, SpaceListPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveSpaceListMember, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.space_lists.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space list {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.space_ids.contains(&payload.space_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} is not a member of Space list {}.", payload.space_id.0, payload.id.0), [payload.space_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { space_lists: Rows::modifying(SpaceListPatch { space_ids: ListEdit::removing_where(&existing.space_ids, |candidate| *candidate == payload.space_id), ..SpaceListPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
