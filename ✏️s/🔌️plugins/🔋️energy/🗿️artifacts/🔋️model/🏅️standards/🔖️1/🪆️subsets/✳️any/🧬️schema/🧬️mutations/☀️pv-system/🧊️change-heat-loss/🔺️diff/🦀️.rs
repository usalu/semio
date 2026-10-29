//! 🔺️ Sparse authored PV thermal field diff.
use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch, PvSystemAssignmentPatch};
use crate::diff_patch::Rows;
use crate::EnergyModelSnapshot;
pub fn diff(payload:&super::ChangePvSystemFaimanHeatLossConstant,base:&EnergyModelSnapshot)->protocol::MutationOutcome<EnergyModelDiff>{
    let Some(existing)=base.model.pv_systems.iter().find(|item|item.id==payload.id) else {return protocol::MutationOutcome::error("mutation.target-missing","PV system does not exist.",[payload.id.0.to_string()]);};
    if !payload.new_faiman_heat_loss_constant_w_m2k.is_finite() || payload.new_faiman_heat_loss_constant_w_m2k <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Constant heat loss violates the physical finite-value contract.", [payload.id.0.to_string()]);
    }
    if existing.faiman_heat_loss_constant_w_m2k==payload.new_faiman_heat_loss_constant_w_m2k{return protocol::MutationOutcome::empty().warning("mutation.no-op","PV system already carries this authored thermal value.");}
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch{pv_systems:Rows::modifying(PvSystemAssignmentPatch{faiman_heat_loss_constant_w_m2k:Some(payload.new_faiman_heat_loss_constant_w_m2k),..PvSystemAssignmentPatch::of(payload.id)}),..Default::default()}))
}
