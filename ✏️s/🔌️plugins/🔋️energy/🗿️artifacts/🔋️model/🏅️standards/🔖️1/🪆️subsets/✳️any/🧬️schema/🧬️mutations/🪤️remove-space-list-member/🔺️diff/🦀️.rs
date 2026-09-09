//! 🔺️ Sparse diff builder for `RemoveSpaceListMember` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveSpaceListMember, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.space_lists.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space list {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.space_ids.contains(&payload.space_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} is not a member of Space list {}.", payload.space_id.0, payload.id.0), [payload.space_id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.space_lists.iter_mut().find(|item| item.id == payload.id) {
        item.space_ids.retain(|candidate| *candidate != payload.space_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
