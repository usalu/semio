//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationFanTotalEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MechanicalVentilationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationFanTotalEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_fan_total_efficiency > 0.0 && payload.new_fan_total_efficiency <= 1.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Mechanical Ventilation {}: fan total efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_fan_total_efficiency), [payload.id.0.to_string()]);
    }
    if existing.fan_total_efficiency == payload.new_fan_total_efficiency {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Mechanical Ventilation {} already carries this fan total efficiency: {}.", payload.id.0, payload.new_fan_total_efficiency));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { mechanical_ventilations: Rows::modifying(MechanicalVentilationPatch { fan_total_efficiency: Some(payload.new_fan_total_efficiency), ..MechanicalVentilationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
