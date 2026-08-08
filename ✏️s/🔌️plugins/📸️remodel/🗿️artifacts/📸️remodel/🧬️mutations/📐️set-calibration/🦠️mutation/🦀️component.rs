//! 📐️ Remodel mutation — `SetCalibration` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, calibration: &crate::artifacts::remodel::CalibrationState) {
    next.calibration = calibration.clone();
}
//#endregion 🔖️Mutation
