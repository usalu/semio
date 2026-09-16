//! 🔺️ Sparse diff builder for `ChangeGlazingMaterialConductivity` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGlazingMaterialConductivity, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.glazing_materials.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Glazing material {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_conductivity_w_m_k.is_finite() || payload.new_conductivity_w_m_k <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Glazing material {}: conductivity (W/m·K) must be a positive finite value, got {}.", payload.id.0, payload.new_conductivity_w_m_k), [payload.id.0.to_string()]);
    }
    if existing.conductivity_w_m_k == payload.new_conductivity_w_m_k {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Glazing material {} already carries this conductivity (W/m·K): {}.", payload.id.0, payload.new_conductivity_w_m_k));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.glazing_materials.iter_mut().find(|item| item.id == payload.id) {
        item.conductivity_w_m_k = payload.new_conductivity_w_m_k;
    }
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
