//! 🔺️ Sparse authored PV thermal field diff.
use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch, PvSystemAssignmentPatch, Rows};
use crate::EnergyModelSnapshot;
pub fn diff(payload:&super::ChangePvSystemMounting,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{
    let Some(existing)=base.model.pv_systems.iter().find(|item|item.id==payload.id) else {return protocol::MutationOutcome::error("mutation.target-missing","PV system does not exist.",[payload.id.0.to_string()]);};
    if existing.mounting==payload.new_mounting{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch{pv_systems:Rows::modifying(PvSystemAssignmentPatch{mounting:Some(payload.new_mounting),..PvSystemAssignmentPatch::of(payload.id)}),..Default::default()}))
}
