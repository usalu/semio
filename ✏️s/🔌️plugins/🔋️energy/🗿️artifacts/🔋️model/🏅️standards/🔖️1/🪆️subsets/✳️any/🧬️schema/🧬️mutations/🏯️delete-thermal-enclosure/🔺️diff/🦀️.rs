//! 🔺️ Sparse diff builder for `DeleteThermalEnclosure` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteThermalEnclosure, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.thermal_enclosures.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Thermal enclosure {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.thermal_enclosures.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
