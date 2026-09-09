//! 🔺️ Sparse diff builder for `ChangeRefrigerationSystemCaseCount` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRefrigerationSystemCaseCount, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Refrigeration system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_case_count == 0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Refrigeration system {} needs at least one display case.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.case_count == payload.new_case_count {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Refrigeration system {} already carries this case_count: {}.", payload.id.0, payload.new_case_count));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.refrigeration_systems.iter_mut().find(|item| item.id == payload.id) {
        item.case_count = payload.new_case_count;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
