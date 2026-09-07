//! 🔺️ Sparse diff builder for `ChangeInfiltrationMethod` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationMethod, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.method == payload.new_method {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this infiltration method: {:?}.", payload.id.0, payload.new_method));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.method = payload.new_method;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
