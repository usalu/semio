//! 🔺️ Sparse diff builder for `ChangeRefrigerationSystemDesignLoad` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RefrigerationConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRefrigerationSystemDesignLoad, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Refrigeration system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !payload.new_design_load_w.is_finite() || payload.new_design_load_w <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Refrigeration system {}: design load (W) must be a positive finite value, got {}.", payload.id.0, payload.new_design_load_w), [payload.id.0.to_string()]);
    }
    if existing.design_load_w == payload.new_design_load_w {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Refrigeration system {} already carries this design load (W): {}.", payload.id.0, payload.new_design_load_w));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { refrigeration_systems: Rows::modifying(RefrigerationConfigPatch { design_load_w: Some(payload.new_design_load_w), ..RefrigerationConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
