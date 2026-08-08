//! 📐️ Remodel mutation — `SetCalibration` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, calibration: &crate::artifacts::remodel::CalibrationState) {
    next.calibration = calibration.clone();
}
//#endregion 🔖️Mutation
