//! ↩️ Inverse for `DisconnectReferencedModel` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DisconnectReferencedModel, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let _ = payload;
    match &base.referenced_model {
        Some(existing) => vec![vocabulary::connect_referenced_model(existing.target.to_uri())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
