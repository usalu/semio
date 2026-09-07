//! 🔺️ Sparse diff builder for `ChangePvSystemModuleEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePvSystemModuleEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_module_efficiency > 0.0 && payload.new_module_efficiency <= 1.0) {
        return protocol::MutationOutcome::error("mutation.invariant", format!("PV system {}: module efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_module_efficiency), [payload.id.0.to_string()]);
    }
    if existing.module_efficiency == payload.new_module_efficiency {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("PV system {} already carries this module efficiency: {}.", payload.id.0, payload.new_module_efficiency));
    }
    let mut model = base.model.clone();
    if let Some(item) = model.pv_systems.iter_mut().find(|item| item.id == payload.id) {
        item.module_efficiency = payload.new_module_efficiency;
    }
    protocol::MutationOutcome::new(crate::artifacts::model::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
