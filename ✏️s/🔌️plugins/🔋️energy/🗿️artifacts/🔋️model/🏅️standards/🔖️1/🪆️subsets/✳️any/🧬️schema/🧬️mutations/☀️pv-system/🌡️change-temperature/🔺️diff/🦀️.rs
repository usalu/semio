//! 🔺️ Sparse authored PV thermal field diff.
use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch, PvSystemAssignmentPatch};
use crate::diff_patch::Rows;
use crate::EnergyModelSnapshot;
pub fn diff(payload:&super::ChangePvSystemTemperatureCoefficient,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{
    let Some(existing)=base.model.pv_systems.iter().find(|item|item.id==payload.id) else {return protocol::MutationOutcome::error("mutation.target-missing","PV system does not exist.",[payload.id.0.to_string()]);};
    if !payload.new_temperature_coefficient.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Temperature coefficient violates the physical finite-value contract.", [payload.id.0.to_string()]);
    }
    if existing.temperature_coefficient==payload.new_temperature_coefficient{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch{pv_systems:Rows::modifying(PvSystemAssignmentPatch{temperature_coefficient:Some(payload.new_temperature_coefficient),..PvSystemAssignmentPatch::of(payload.id)}),..Default::default()}))
}
