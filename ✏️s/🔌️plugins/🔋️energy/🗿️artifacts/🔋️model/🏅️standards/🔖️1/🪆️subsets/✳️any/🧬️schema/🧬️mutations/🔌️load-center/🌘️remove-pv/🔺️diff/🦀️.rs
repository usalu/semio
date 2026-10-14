//! 🔺️ Sparse diff builder for `RemoveElectricalLoadCenterPv` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ElectricalLoadCenterPatch, ListEdit, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveElectricalLoadCenterPv, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.electrical_load_centers.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Electrical load center {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !existing.pv_ids.contains(&payload.pv_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} is not a member of Electrical load center {}.", payload.pv_id.0, payload.id.0), [payload.pv_id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { electrical_load_centers: Rows::modifying(ElectricalLoadCenterPatch { pv_ids: ListEdit::removing_where(&existing.pv_ids, |candidate| *candidate == payload.pv_id), ..ElectricalLoadCenterPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
