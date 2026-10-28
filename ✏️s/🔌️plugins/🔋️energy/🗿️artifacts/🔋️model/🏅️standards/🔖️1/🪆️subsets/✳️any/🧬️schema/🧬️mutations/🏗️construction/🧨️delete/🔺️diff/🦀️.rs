//! 🔺️ Sparse diff builder for `DeleteConstruction` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch};
use crate::diff_patch::{Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteConstruction, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.constructions.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Construction {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;
    if base.model.surfaces.iter().any(|surface| surface.construction_id == payload.id)
        || base.model.fenestrations.iter().any(|fenestration| fenestration.glazing_construction_id == Some(payload.id)) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Construction {} is still assigned to a surface or fenestration.", payload.id.0), [payload.id.0.to_string()]);
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { constructions: Rows::removing(&base.model.constructions, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
