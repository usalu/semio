//! 🔺️ Sparse diff builder for `ChangeRefrigerationSystemCaseCount` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, ModelPatch, RefrigerationConfigPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeRefrigerationSystemCaseCount, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.refrigeration_systems.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Refrigeration system {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if payload.new_case_count == 0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Refrigeration system {} needs at least one display case.", payload.id.0), [payload.id.0.to_string()]);
    }
    if existing.case_count == payload.new_case_count {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Refrigeration system {} already carries this case_count: {}.", payload.id.0, payload.new_case_count));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { refrigeration_systems: Rows::modifying(RefrigerationConfigPatch { case_count: Some(payload.new_case_count), ..RefrigerationConfigPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
