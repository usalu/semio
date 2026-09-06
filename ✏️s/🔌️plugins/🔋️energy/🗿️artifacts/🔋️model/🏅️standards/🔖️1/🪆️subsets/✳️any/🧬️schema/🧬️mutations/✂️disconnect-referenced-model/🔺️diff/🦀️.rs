//! 🔺️ Sparse diff builder for `DisconnectReferencedModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::artifacts::model::EnergyModelSnapshot;
use crate::artifacts::model::diff::EnergyLinkSlotDelta;
use crate::artifacts::model::diff::EnergyModelDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::DisconnectReferencedModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    let Some(existing) = &base.referenced_model else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No referenced model is connected to this energy model.", Vec::<String>::new());
    };
    let _ = existing;
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })
}
//#endregion 🔖️Diff
