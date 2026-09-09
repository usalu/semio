//! 🔺️ Sparse diff builder for `ChangeFenestrationVlt` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationVlt, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_vlt.is_finite() || !(0.0..=1.0).contains(&payload.new_vlt) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Fenestration {} needs a visible transmittance in 0..=1, got {}.", payload.id.0, payload.new_vlt), [payload.id.0.to_string()]);
    }
    if existing.vlt == payload.new_vlt {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Fenestration {} already has this visible transmittance.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.fenestrations.iter_mut().find(|item| item.id == payload.id) {
        item.vlt = payload.new_vlt;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
