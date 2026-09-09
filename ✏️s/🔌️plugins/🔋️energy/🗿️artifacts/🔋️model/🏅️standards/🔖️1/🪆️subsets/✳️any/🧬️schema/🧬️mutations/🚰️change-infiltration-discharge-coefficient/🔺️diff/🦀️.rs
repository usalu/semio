//! 🔺️ Sparse diff builder for `ChangeInfiltrationDischargeCoefficient` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationDischargeCoefficient, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_discharge_coefficient.is_finite() || payload.new_discharge_coefficient <= 0.0 {
        return protocol::MutationOutcome::error(
            "mutation.invariant",
            format!("Infiltration {}: the orifice discharge coefficient must be a positive finite value, got {}.", payload.id.0, payload.new_discharge_coefficient),
            [payload.id.0.to_string()],
        );
    }
    if existing.discharge_coefficient == payload.new_discharge_coefficient {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this the orifice discharge coefficient: {}.", payload.id.0, payload.new_discharge_coefficient));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.discharge_coefficient = payload.new_discharge_coefficient;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
