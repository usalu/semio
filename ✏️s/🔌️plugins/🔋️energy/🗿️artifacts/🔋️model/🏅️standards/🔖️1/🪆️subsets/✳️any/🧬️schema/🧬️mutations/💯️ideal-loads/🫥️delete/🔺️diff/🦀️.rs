//! 🔺️ Sparse diff builder for `DeleteIdealLoadsSystem` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch};
use crate::diff_patch::{Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteIdealLoadsSystem, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.ideal_loads.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Ideal loads system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.faults.iter().any(|fault| fault.target_equipment_id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-referenced", "The ideal loads system is still referenced by a fault.", [payload.id.0.to_string()]);
    }

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ideal_loads: Rows::removing(&base.model.ideal_loads, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
