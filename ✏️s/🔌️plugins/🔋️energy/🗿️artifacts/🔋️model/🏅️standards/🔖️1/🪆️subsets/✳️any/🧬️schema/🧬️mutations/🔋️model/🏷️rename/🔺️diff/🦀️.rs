//! 🔺️ Sparse diff builder for `RenameModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An energy model name must not be blank.", [payload.new_name.clone()]);
    }
    if base.model.name == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The energy model is already named \"{}\".", payload.new_name));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { name: Some(payload.new_name.clone()), ..Default::default() }))
}
//#endregion 🔖️Diff
