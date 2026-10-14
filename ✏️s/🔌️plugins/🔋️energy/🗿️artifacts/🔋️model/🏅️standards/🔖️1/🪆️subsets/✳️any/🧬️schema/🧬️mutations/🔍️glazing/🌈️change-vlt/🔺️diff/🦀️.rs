//! 🔺️ Sparse diff builder for `ChangeFenestrationVlt` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationVlt, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_vlt.is_finite() || !(0.0..=1.0).contains(&payload.new_vlt) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fenestration {} needs a visible transmittance in 0..=1, got {}.", payload.id.0, payload.new_vlt), [payload.id.0.to_string()]);
    }
    if existing.vlt == payload.new_vlt {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} already has this visible transmittance.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { vlt: Some(payload.new_vlt), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
