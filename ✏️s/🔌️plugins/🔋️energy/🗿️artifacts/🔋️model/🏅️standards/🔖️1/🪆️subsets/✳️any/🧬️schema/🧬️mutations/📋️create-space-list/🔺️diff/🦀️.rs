//! 🔺️ Sparse diff builder for `CreateSpaceList` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSpaceList, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.space_lists.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.duplicate-id", format!("Space list {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if payload.index as usize > base.model.space_lists.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Index {} is past the end of the model's {} space_lists.", payload.index, base.model.space_lists.len()), [payload.id.0.to_string()]);
    }
    if let Some(missing) = payload.space_ids.iter().find(|candidate| !base.model.spaces.iter().any(|row| row.id == **candidate)) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", missing.0), [missing.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.space_lists.insert(payload.index as usize, crate::model::SpaceList { id: payload.id, name: payload.name.clone(), space_ids: payload.space_ids.clone() });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
