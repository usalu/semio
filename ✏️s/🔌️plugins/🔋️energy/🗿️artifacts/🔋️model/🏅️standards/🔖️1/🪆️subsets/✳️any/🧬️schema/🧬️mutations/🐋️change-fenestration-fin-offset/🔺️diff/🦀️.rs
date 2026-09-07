//! 🔺️ Sparse diff builder for `ChangeFenestrationFinOffset` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationFinOffset, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_fin_offset_m.is_finite() || payload.new_fin_offset_m < 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a non-negative finite fin offset, got {}.", payload.id.0, payload.new_fin_offset_m), [payload.id.0.to_string()]);
    }
    if existing.fin_offset_m == payload.new_fin_offset_m {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this fin offset.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.fin_offset_m = payload.new_fin_offset_m;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
