//! 🔺️ Sparse diff builder for `AddOutputVariable` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, OutputVariableSpecPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::AddOutputVariable, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if payload.name.trim().is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "An output variable name must not be blank.", [payload.key.clone()]);
    }
    if base.model.output_variables.iter().any(|spec| spec.name == payload.name && spec.key == payload.key) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Output variable \"{}\" is already registered for \"{}\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    }
    let position = payload.index.map_or(base.model.output_variables.len(), |index| index as usize);
    if position > base.model.output_variables.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} output_variables.", position, base.model.output_variables.len()), [payload.name.clone(), payload.key.clone()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { output_variables: Rows::inserting(position, crate::model::OutputVariableSpec { name: payload.name.clone(), key: payload.key.clone(), reporting_frequency: payload.reporting_frequency }), ..Default::default() }))
}
//#endregion 🔖️Diff
