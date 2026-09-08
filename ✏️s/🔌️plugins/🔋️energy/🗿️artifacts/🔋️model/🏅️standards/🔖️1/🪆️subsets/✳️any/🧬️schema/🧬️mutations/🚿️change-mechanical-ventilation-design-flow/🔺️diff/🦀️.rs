//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationDesignFlow` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationDesignFlow, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_flow_m3_s.is_finite() || payload.new_design_flow_m3_s < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Mechanical Ventilation {}: design supply flow (m³/s) must be a finite non-negative value, got {}.", payload.id.0, payload.new_design_flow_m3_s), [payload.id.0.to_string()]);
    }
    if existing.design_flow_m3_s == payload.new_design_flow_m3_s {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Mechanical Ventilation {} already carries this design supply flow (m³/s): {}.", payload.id.0, payload.new_design_flow_m3_s));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.mechanical_ventilations.iter_mut().find(|item| item.id == payload.id) {
        item.design_flow_m3_s = payload.new_design_flow_m3_s;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
