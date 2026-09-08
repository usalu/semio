//! 🔺️ Sparse diff builder for `DeleteConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.surfaces.iter().any(|surface| surface.construction_id == payload.id) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Construction {} is still assigned to a surface.", payload.id.0), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    model.constructions.retain(|item| item.id != payload.id);
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
