//! ↩️ Inverse for `ChangeModelVersion` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeModelVersion, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.new_version.trim().is_empty() || base.model.version == payload.new_version {
        return Vec::new();
    }
    vec![vocabulary::change_model_version(base.model.version.clone())]
}
//#endregion 🔖️Inverse
