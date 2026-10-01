//! 🔺️ Sparse diff builder for `RemoveAnnualScheduleRule` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAnnualScheduleRule, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.index as usize >= existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} has no rule at index {}.", payload.id.0, payload.index), [payload.id.0.to_string()]);
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        item.rules.remove(payload.index as usize);
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
