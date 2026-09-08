//! 🔺️ Sparse diff builder for `DeleteSpace` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSpace, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.space_lists.iter().any(|item| item.space_ids.contains(&payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Space {} is still a member of a space list.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.spaces.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
