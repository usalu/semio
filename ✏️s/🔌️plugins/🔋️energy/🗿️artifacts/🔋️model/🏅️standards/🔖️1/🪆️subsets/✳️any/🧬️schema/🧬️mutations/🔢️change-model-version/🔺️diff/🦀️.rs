//! 🔺️ Sparse diff builder for `ChangeModelVersion` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeModelVersion, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.new_version.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An energy model version must not be blank.", [payload.new_version.clone()]);
    }
    if base.model.version == payload.new_version {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The energy model version is already \"{}\".", payload.new_version));
    }
    let mut model = base.model.clone();
    model.version = payload.new_version.clone();
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
