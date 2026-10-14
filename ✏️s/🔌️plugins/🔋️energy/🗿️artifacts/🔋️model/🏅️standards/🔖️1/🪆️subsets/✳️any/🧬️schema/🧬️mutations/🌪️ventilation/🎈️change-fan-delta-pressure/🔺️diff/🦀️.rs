//! 🔺️ Sparse diff builder for `ChangeMechanicalVentilationFanDeltaPressure` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, MechanicalVentilationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeMechanicalVentilationFanDeltaPressure, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.mechanical_ventilations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Mechanical Ventilation {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_fan_delta_pressure_pa.is_finite() || payload.new_fan_delta_pressure_pa < 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Mechanical Ventilation {}: fan pressure rise (Pa) must be a finite non-negative value, got {}.", payload.id.0, payload.new_fan_delta_pressure_pa),
            [payload.id.0.to_string()],
        );
    }
    if existing.fan_delta_pressure_pa == payload.new_fan_delta_pressure_pa {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Mechanical Ventilation {} already carries this fan pressure rise (Pa): {}.", payload.id.0, payload.new_fan_delta_pressure_pa));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { mechanical_ventilations: Rows::modifying(MechanicalVentilationPatch { fan_delta_pressure_pa: Some(payload.new_fan_delta_pressure_pa), ..MechanicalVentilationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
