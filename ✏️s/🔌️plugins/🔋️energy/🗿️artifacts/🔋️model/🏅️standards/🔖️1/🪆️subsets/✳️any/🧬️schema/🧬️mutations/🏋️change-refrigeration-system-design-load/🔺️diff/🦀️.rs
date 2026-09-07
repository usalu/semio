//! 🔺️ Sparse diff builder for `ChangeRefrigerationSystemDesignLoad` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRefrigerationSystemDesignLoad, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Refrigeration system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_load_w.is_finite() || payload.new_design_load_w <= 0.0 {
        return protocol::MutationOutcome::error("mutation.invariant", format!("Refrigeration system {}: design load (W) must be a positive finite value, got {}.", payload.id.0, payload.new_design_load_w), [payload.id.0.to_string()]);
    }
    if existing.design_load_w == payload.new_design_load_w {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Refrigeration system {} already carries this design load (W): {}.", payload.id.0, payload.new_design_load_w));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.refrigeration_systems.iter_mut().find(|item| item.id == payload.id) {
        item.design_load_w = payload.new_design_load_w;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
