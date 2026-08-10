//! ↩️ Inverse for `SetGcps`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetGcps { gcps: base.gcps.clone() }]
}
//#endregion 🔖️Inverse
