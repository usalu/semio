//! 🔺️ Sparse diff builder for `RemoveOutputVariable` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch, OutputVariableSpecPatch};
use crate::diff_patch::{Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveOutputVariable, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.output_variables.iter().find(|spec| spec.name == payload.name && spec.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Output variable \"{}\" is not registered for \"{}\".", payload.name, payload.key), [payload.name.clone(), payload.key.clone()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { output_variables: Rows::<OutputVariableSpecPatch>::removing_where(&base.model.output_variables, |spec| spec.name == payload.name && spec.key == payload.key), ..Default::default() }))
}
//#endregion 🔖️Diff
