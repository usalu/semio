//! 🔺️ Sparse diff builder for `ChangeSizingObjectDesignDayType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, Rows, SizingObjectPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSizingObjectDesignDayType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.sizing_objects.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Sizing object {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.design_day_type == payload.new_design_day_type {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Sizing object {} already has that design day type.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { sizing_objects: Rows::modifying(SizingObjectPatch { design_day_type: Some(payload.new_design_day_type), ..SizingObjectPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
