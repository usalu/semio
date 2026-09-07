//! 🔺️ Sparse diff builder for `ReorderAnnualScheduleRules` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ReorderAnnualScheduleRules, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.schedules.annual.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Annual schedule {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.from as usize >= existing.rules.len() || payload.to as usize >= existing.rules.len() {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Annual schedule {} has {} rules, so {} .. {} is not a move.", payload.id.0, existing.rules.len(), payload.from, payload.to), [payload.id.0.to_string()]);
    }
    if payload.from == payload.to {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Annual schedule {} rule {} is already at that position.", payload.id.0, payload.from));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.schedules.annual.iter_mut().find(|item| item.id == payload.id) {
        let rule = item.rules.remove(payload.from as usize);
        item.rules.insert(payload.to as usize, rule);
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
