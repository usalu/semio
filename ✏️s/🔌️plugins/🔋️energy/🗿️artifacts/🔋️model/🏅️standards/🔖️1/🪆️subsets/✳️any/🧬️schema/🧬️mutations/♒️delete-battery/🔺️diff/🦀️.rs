//! 🔺️ Sparse diff builder for `DeleteBattery` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteBattery, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.battery_storage.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Battery {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.electrical_load_centers.iter().any(|centre| centre.battery_ids.contains(&payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Battery {} is still attached to an electrical load centre.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.battery_storage.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
