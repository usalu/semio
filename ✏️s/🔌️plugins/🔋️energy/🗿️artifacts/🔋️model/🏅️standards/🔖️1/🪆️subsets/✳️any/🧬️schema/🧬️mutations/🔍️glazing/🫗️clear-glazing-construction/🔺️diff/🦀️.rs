//! 🔺️ Sparse diff builder for `ClearFenestrationGlazingConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, FenestrationPatch, ModelPatch};
use crate::diff_patch::{OptionChange, Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ClearFenestrationGlazingConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if existing.glazing_construction_id.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} has no glazing construction to clear.", payload.id.0), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { glazing_construction_id: OptionChange::assign(None), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
