//! ↩️ Inverse for `SetCalibration`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetCalibration { calibration: base.calibration.clone() }]
}
//#endregion 🔖️Inverse
