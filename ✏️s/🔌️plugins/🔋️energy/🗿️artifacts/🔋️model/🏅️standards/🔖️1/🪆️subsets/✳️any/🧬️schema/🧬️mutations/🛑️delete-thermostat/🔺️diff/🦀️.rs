//! 🔺️ Sparse diff builder for `DeleteThermostat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteThermostat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermostats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermostat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    let mut model = base.model.clone();
    model.thermostats.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
