/** 🔺 update-camera-calibration diff — populates `RemodelDiff.calibration`, or nothing if absent. */
export interface UpdateCameraCalibrationDiff {
  calibration?: { cameras: unknown[]; rig: unknown[] };
}
