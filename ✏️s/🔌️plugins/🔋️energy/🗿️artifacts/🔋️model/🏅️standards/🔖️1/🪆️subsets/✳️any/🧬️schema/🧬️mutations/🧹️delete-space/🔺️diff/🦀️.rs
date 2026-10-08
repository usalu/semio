//! 🔺️ Sparse diff builder for `DeleteSpace` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SpacePatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteSpace, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Space {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.space_lists.iter().any(|item| item.space_ids.contains(&payload.id)) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Space {} is still a member of a space list.", payload.id.0), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { spaces: Rows::removing(&base.model.spaces, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
