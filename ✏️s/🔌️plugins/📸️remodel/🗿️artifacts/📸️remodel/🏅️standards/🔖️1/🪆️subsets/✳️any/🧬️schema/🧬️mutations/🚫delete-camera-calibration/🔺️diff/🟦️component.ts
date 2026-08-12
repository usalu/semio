/** 🔺 delete-camera-calibration diff — populates `RemodelDiff.calibration`, or nothing if absent. */
export interface DeleteCameraCalibrationDiff {
  calibration?: { cameras: unknown[]; rig: unknown[] };
}
