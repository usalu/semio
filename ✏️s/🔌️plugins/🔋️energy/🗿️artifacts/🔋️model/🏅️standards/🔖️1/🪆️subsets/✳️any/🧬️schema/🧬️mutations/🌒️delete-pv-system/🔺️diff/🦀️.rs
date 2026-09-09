//! 🔺️ Sparse diff builder for `DeletePvSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeletePvSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.electrical_load_centers.iter().any(|centre| centre.pv_ids.contains(&payload.id)) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("PV system {} is still attached to an electrical load centre.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.pv_systems.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
