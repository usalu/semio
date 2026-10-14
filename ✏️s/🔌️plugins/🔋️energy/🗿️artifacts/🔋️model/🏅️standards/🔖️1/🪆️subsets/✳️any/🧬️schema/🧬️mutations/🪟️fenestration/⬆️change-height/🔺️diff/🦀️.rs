//! 🔺️ Sparse diff builder for `ChangeFenestrationHeight` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeFenestrationHeight, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_height_m.is_finite() || payload.new_height_m <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Fenestration {} needs a positive finite height, got {}.", payload.id.0, payload.new_height_m), [payload.id.0.to_string()]);
    }
    if existing.height_m == payload.new_height_m {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} already has this height.", payload.id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { height_m: Some(payload.new_height_m), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
