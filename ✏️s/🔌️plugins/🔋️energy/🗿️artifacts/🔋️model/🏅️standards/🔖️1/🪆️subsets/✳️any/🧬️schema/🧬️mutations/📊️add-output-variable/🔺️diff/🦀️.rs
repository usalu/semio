//! 🔺️ Sparse diff builder for `AddOutputVariable` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::AddOutputVariable, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::error("mutation.invariant", "An output variable name must not be blank.", [payload.key.clone()]);
    }
    if base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.duplicate", format!("Output variable \"{}\" is already registered for \"{}\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    }
    let mut model = base.model.clone();
    model.output_variables.push(crate::model::OutputVariableSpec { name: payload.name.clone(), key: payload.key.clone(), reporting_frequency: payload.reporting_frequency });
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
