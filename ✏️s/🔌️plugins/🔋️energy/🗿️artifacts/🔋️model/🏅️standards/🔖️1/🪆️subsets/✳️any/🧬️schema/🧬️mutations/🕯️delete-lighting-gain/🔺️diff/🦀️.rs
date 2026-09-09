//! 🔺️ Sparse diff builder for `DeleteLightingGain` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteLightingGain, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.lighting.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Lighting Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.lighting.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
