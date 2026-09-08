//! 🔺️ Sparse diff builder for `ChangeLightingGainVisibleFraction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::EnergyModelSnapshot;
use crate::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeLightingGainVisibleFraction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.lighting.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Lighting Gain {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(0.0..=1.0).contains(&payload.new_visible_fraction) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Lighting Gain {}: visible fraction must be a fraction in [0, 1], got {}.", payload.id.0, payload.new_visible_fraction), [payload.id.0.to_string()]);
    }
    if existing.visible_fraction == payload.new_visible_fraction {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Lighting Gain {} already carries this visible fraction: {}.", payload.id.0, payload.new_visible_fraction));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.lighting.iter_mut().find(|item| item.id == payload.id) {
        item.visible_fraction = payload.new_visible_fraction;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
