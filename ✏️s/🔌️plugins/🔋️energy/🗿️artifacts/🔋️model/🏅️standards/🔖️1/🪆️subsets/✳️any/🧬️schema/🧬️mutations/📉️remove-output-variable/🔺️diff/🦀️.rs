//! 🔺️ Sparse diff builder for `RemoveOutputVariable` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveOutputVariable, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.output_variables.iter().find(|spec| spec.name == payload.name && spec.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Output variable \"{}\" is not registered for \"{}\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.output_variables.retain(|spec| !(spec.name == payload.name && spec.key == payload.key));
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
