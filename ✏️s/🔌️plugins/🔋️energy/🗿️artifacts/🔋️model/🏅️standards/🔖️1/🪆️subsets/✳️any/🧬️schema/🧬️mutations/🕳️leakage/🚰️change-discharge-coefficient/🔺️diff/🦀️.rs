//! 🔺️ Sparse diff builder for `ChangeInfiltrationDischargeCoefficient` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, InfiltrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationDischargeCoefficient, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_discharge_coefficient.is_finite() || payload.new_discharge_coefficient <= 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Infiltration {}: the orifice discharge coefficient must be a positive finite value, got {}.", payload.id.0, payload.new_discharge_coefficient),
            [payload.id.0.to_string()],
        );
    }
    if existing.discharge_coefficient == payload.new_discharge_coefficient {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Infiltration {} already carries this the orifice discharge coefficient: {}.", payload.id.0, payload.new_discharge_coefficient));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { infiltrations: Rows::modifying(InfiltrationPatch { discharge_coefficient: Some(payload.new_discharge_coefficient), ..InfiltrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
