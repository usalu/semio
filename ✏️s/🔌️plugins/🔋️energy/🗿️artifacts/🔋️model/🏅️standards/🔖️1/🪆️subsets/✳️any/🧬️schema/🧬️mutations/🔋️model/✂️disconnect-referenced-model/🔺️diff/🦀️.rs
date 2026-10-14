//! 🔺️ Sparse diff builder for `DisconnectReferencedModel` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyLinkSlotDelta;
use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(_payload: &super::DisconnectReferencedModel, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if base.referenced_model.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "No referenced model is connected to this energy model.", Vec::<String>::new());
    }
    protocol::MutationOutcome::new(EnergyModelDiff { referenced_model: Some(EnergyLinkSlotDelta::Detached), ..Default::default() })
}
//#endregion 🔖️Diff
