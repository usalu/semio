//! 🔺️ Sparse diff builder for `ChangeFaultType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FaultDefinitionPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFaultType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.faults.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fault {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.fault_type == payload.new_fault_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fault {} already carries this fault_type: {:?}.", payload.id.0, payload.new_fault_type));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { faults: Rows::modifying(FaultDefinitionPatch { fault_type: Some(payload.new_fault_type), ..FaultDefinitionPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
