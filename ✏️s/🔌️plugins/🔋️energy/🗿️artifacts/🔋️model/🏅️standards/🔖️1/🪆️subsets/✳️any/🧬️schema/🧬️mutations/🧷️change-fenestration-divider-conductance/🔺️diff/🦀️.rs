//! 🔺️ Sparse diff builder for `ChangeFenestrationDividerConductance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationDividerConductance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_divider_conductance_w_k.is_finite() || payload.new_divider_conductance_w_k < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a non-negative finite divider conductance, got {}.", payload.id.0, payload.new_divider_conductance_w_k), [payload.id.0.to_string()]);
    }
    if existing.divider_conductance_w_k == payload.new_divider_conductance_w_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this divider conductance.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.divider_conductance_w_k = payload.new_divider_conductance_w_k;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
