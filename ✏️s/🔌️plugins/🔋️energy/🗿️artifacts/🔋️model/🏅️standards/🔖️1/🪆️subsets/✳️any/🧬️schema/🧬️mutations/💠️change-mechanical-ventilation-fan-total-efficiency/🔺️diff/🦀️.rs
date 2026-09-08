//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationFanTotalEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationFanTotalEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_fan_total_efficiency > 0.0 && payload.new_fan_total_efficiency <= 1.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Mechanical Ventilation {}: fan total efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_fan_total_efficiency), [payload.id.0.to_string()]);
    }
    if existing.fan_total_efficiency == payload.new_fan_total_efficiency {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Mechanical Ventilation {} already carries this fan total efficiency: {}.", payload.id.0, payload.new_fan_total_efficiency));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.mechanical_ventilations.iter_mut().find(|item| item.id == payload.id) {
        item.fan_total_efficiency = payload.new_fan_total_efficiency;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
