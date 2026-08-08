//! ↩️ Inverse for `SetGcps`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetGcps { gcps: base.gcps.clone() }]
}
//#endregion 🔖️Inverse
