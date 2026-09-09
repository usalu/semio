//! 🔺️ Sparse diff builder for `ChangeInfiltrationStackHeight` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationStackHeight, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_stack_height_m.is_finite() || payload.new_stack_height_m < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Infiltration {}: stack height (m) must be a finite non-negative value, got {}.", payload.id.0, payload.new_stack_height_m), [payload.id.0.to_string()]);
    }
    if existing.stack_height_m == payload.new_stack_height_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Infiltration {} already carries this stack height (m): {}.", payload.id.0, payload.new_stack_height_m));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.infiltrations.iter_mut().find(|item| item.id == payload.id) {
        item.stack_height_m = payload.new_stack_height_m;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
