//! 🔺️ Sparse diff builder for `ChangeSizingObjectDesignDayType` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeSizingObjectDesignDayType, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.sizing_objects.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Sizing object {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };

    if existing.design_day_type == payload.new_design_day_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Sizing object {} already has that design day type.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.sizing_objects.iter_mut().find(|item| item.id == payload.id) {
        item.design_day_type = payload.new_design_day_type;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
