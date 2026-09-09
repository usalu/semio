//! 🔺️ Sparse diff builder for `RenameModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenameModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.new_name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An energy model name must not be blank.", [payload.new_name.clone()]);
    }
    if base.model.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The energy model is already named \"{}\".", payload.new_name));
    }
    let mut model = base.model.clone();
    model.name = payload.new_name.clone();
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
