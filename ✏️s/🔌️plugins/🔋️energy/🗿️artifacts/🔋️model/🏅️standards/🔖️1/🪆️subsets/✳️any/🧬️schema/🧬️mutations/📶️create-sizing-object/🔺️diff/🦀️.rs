//! 🔺️ Sparse diff builder for `CreateSizingObject` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SizingObjectPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreateSizingObject, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.model.sizing_objects.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Sizing object {} already exists.", payload.id.0), [payload.id.0.to_string()]);
    }
    if !base.model.zones.iter().any(|zone| zone.id == payload.zone_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Zone {} does not exist.", payload.zone_id.0), [payload.zone_id.0.to_string()]);
    }
    let position = payload.index.map_or(base.model.sizing_objects.len(), |index| index as usize);
    if position > base.model.sizing_objects.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Index {} is past the end of the model's {} sizing_objects.", position, base.model.sizing_objects.len()), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { sizing_objects: Rows::inserting(position, crate::model::SizingObject { id: payload.id, zone_id: payload.zone_id, sizing_type: payload.sizing_type, design_day_type: payload.design_day_type }), ..Default::default() }))
}
//#endregion 🔖️Diff
