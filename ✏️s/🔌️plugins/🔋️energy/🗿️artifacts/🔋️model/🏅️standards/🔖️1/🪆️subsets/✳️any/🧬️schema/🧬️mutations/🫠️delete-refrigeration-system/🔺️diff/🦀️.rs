//! 🔺️ Sparse diff builder for `DeleteRefrigerationSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRefrigerationSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Refrigeration system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    let mut model = base.model.clone();
    model.refrigeration_systems.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
