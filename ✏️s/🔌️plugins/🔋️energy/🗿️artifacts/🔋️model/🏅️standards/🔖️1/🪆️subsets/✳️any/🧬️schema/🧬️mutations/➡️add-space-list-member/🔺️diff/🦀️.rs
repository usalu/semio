//! 🔺️ Sparse diff builder for `AddSpaceListMember` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddSpaceListMember, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.space_lists.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space list {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.spaces.iter().any(|row| row.id == payload.space_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.space_id.0), [payload.space_id.0.to_string()]);
    }
    if payload.index as usize > existing.space_ids.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of Space list {}'s {} members.", payload.index, payload.id.0, existing.space_ids.len()), [payload.id.0.to_string()]);
    }
    if existing.space_ids.contains(&payload.space_id) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Space {} already belongs to Space list {}.", payload.space_id.0, payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.space_lists.iter_mut().find(|item| item.id == payload.id) {
        item.space_ids.insert(payload.index as usize, payload.space_id);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
