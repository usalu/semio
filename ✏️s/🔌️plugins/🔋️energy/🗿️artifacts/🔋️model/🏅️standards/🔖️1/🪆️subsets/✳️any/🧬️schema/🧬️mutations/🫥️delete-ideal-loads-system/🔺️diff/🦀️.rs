//! 🔺️ Sparse diff builder for `DeleteIdealLoadsSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, IdealLoadsSystemPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteIdealLoadsSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::removing(&base.model.ideal_loads, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
