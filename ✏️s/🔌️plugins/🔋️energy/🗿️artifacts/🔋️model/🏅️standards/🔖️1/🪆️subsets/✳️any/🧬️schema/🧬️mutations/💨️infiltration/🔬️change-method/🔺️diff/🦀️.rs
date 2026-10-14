//! 🔺️ Sparse diff builder for `ChangeInfiltrationMethod` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, InfiltrationPatch, ModelPatch, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeInfiltrationMethod, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.infiltrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Infiltration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.method == payload.new_method {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Infiltration {} already carries this infiltration method: {:?}.", payload.id.0, payload.new_method));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { infiltrations: Rows::modifying(InfiltrationPatch { method: Some(payload.new_method), ..InfiltrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
