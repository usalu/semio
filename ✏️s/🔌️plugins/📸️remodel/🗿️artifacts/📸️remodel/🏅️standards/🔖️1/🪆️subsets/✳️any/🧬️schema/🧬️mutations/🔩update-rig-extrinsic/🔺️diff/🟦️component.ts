/** 🔺 update-rig-extrinsic diff — populates `RemodelDiff.calibration`, or nothing if absent. */
export interface UpdateRigExtrinsicDiff {
  calibration?: { cameras: unknown[]; rig: unknown[] };
}
