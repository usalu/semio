//! 🔺️ Sparse diff builder for `DeleteHumidistat` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{ReferenceEnergyModelDiff as EnergyModelDiff, ModelPatch};
use crate::diff_patch::{Rows};
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteHumidistat, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = base.model.humidistats.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Humidistat {} does not exist.", payload.id.0), [payload.id.0.to_string()]);
    };
    let _ = existing;

    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { humidistats: Rows::removing(&base.model.humidistats, &payload.id), ..Default::default() }))
}
//#endregion 🔖️Diff
