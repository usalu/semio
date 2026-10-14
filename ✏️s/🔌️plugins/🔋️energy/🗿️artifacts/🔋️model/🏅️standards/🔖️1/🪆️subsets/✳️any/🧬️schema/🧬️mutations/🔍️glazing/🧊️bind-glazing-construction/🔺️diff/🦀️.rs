//! 🔺️ Sparse diff builder for `BindFenestrationGlazingConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, FenestrationPatch, ModelPatch, OptionChange, Rows};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::BindFenestrationGlazingConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Fenestration {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    if !base.model.constructions.iter().any(|item| item.id == payload.construction_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.construction_id.0), [payload.id.0.to_string()]);
    }
    if existing.glazing_construction_id == Some(payload.construction_id) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Fenestration {} is already glazed with construction {}.", payload.id.0, payload.construction_id.0));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { fenestrations: Rows::modifying(FenestrationPatch { glazing_construction_id: OptionChange::assign(Some(payload.construction_id)), ..FenestrationPatch::of(payload.id) }), ..Default::default() }))
}
//#endregion 🔖️Diff
