//! 🔺️ Sparse diff builder for `ChangeInfiltrationVelocitySquaredTermCoefficient` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationVelocitySquaredTermCoefficient, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_velocity_squared_term_coefficient.is_finite() || payload.new_velocity_squared_term_coefficient < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Infiltration {}: the squared wind velocity term coefficient D must be a finite non-negative value, got {}.", payload.id.0, payload.new_velocity_squared_term_coefficient), [payload.id.0.to_string()]);
    }
    if existing.velocity_squared_term_coefficient == payload.new_velocity_squared_term_coefficient {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this the squared wind velocity term coefficient D: {}.", payload.id.0, payload.new_velocity_squared_term_coefficient));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.velocity_squared_term_coefficient = payload.new_velocity_squared_term_coefficient;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
