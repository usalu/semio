//! 🔺️ Sparse diff builder for `ChangeFenestrationUValue` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationUValue, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_u_value_w_m2k.is_finite() || payload.new_u_value_w_m2k <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a positive finite U-value, got {}.", payload.id.0, payload.new_u_value_w_m2k), [payload.id.0.to_string()]);
    }
    if existing.u_value_w_m2k == payload.new_u_value_w_m2k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this U-value.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.u_value_w_m2k = payload.new_u_value_w_m2k;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
