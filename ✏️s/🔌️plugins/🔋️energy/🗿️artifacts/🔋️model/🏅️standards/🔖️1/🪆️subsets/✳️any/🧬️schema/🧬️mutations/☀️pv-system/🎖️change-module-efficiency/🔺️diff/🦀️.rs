//! 🔺️ Sparse diff builder for `ChangePvSystemModuleEfficiency` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, PvSystemAssignmentPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePvSystemModuleEfficiency, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.pv_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("PV system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !(payload.new_module_efficiency > 0.0 && payload.new_module_efficiency <= 1.0) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("PV system {}: module efficiency must be a fraction in (0, 1], got {}.", payload.id.0, payload.new_module_efficiency), [payload.id.0.to_string()]);
    }
    if existing.module_efficiency == payload.new_module_efficiency {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("PV system {} already carries this module efficiency: {}.", payload.id.0, payload.new_module_efficiency));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { pv_systems: Rows::modifying(PvSystemAssignmentPatch { module_efficiency: Some(payload.new_module_efficiency), ..PvSystemAssignmentPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
