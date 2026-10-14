//! ↩️ Inverse for `ChangeModelVersion` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeModelVersion, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeModelVersion>,
{
    Ok((|| {
    if payload.new_version.trim().is_empty() || base.model.version == payload.new_version {
        return Vec::new();
    }
    vec![vocabulary::change_model_version(base.model.version.clone())]

    })())
}
//#endregion 🔖️Inverse
