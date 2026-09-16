//! 🔺️ Sparse diff builder for `ChangeGlazingMaterialVisibleTransmittance` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGlazingMaterialVisibleTransmittance, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.glazing_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Glazing material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_visible_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.new_visible_transmittance) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Glazing material {}: visible transmittance must be a finite value in 0..=1, got {}.", payload.id.0, payload.new_visible_transmittance), [payload.id.0.to_string()]);
    }
    if existing.visible_transmittance == payload.new_visible_transmittance {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Glazing material {} already carries this visible transmittance: {}.", payload.id.0, payload.new_visible_transmittance));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.glazing_materials.iter_mut().find(|item| item.id == payload.id) {
        item.visible_transmittance = payload.new_visible_transmittance;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
