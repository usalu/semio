//! 🔺️ Sparse diff builder for `ChangeLightingGainRadiantFraction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, LightingGainPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeLightingGainRadiantFraction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.lighting.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Lighting Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_radiant_fraction) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Lighting Gain {}: radiant fraction must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_radiant_fraction), [payload.id.0.to_string()]);
    }
    if existing.radiant_fraction == payload.new_radiant_fraction {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Lighting Gain {} already carries this radiant fraction: {}.", payload.id.0, payload.new_radiant_fraction));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { lighting: Rows::modifying(LightingGainPatch { radiant_fraction: Some(payload.new_radiant_fraction), ..LightingGainPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
