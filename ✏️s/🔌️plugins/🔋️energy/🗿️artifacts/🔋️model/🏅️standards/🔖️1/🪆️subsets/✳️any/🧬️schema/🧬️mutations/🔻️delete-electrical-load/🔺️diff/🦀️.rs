//! 🔺️ Sparse diff builder for `DeleteElectricalLoadCenter` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ElectricalLoadCenterPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteElectricalLoadCenter, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { electrical_load_centers: Rows::removing(&base.model.electrical_load_centers, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
