//! ↩️ Inverse for `SetCalibration`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetCalibration { calibration: base.calibration.clone() }]
}
//#endregion 🔖️Inverse
