//! 🔺️ Sparse diff builder for `ChangeGlazingMaterialInfraredEmissivity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGlazingMaterialInfraredEmissivity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.glazing_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Glazing material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    for (label, value) in [("front", payload.new_infrared_emissivity_front), ("back", payload.new_infrared_emissivity_back)] {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return protocol::MutationOutcome::error("mutation.invariant", format!("Glazing material {}: the {label} infrared emissivity must be a finite value in 0..=1, got {value}.", payload.id.0), [payload.id.0.to_string()]);
        }
    }
    if existing.infrared_emissivity_front == payload.new_infrared_emissivity_front && existing.infrared_emissivity_back == payload.new_infrared_emissivity_back {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Glazing material {} already carries these infrared emissivities.", payload.id.0));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.glazing_materials.iter_mut().find(|item| item.id == payload.id) {
        item.infrared_emissivity_front = payload.new_infrared_emissivity_front;
        item.infrared_emissivity_back = payload.new_infrared_emissivity_back;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
