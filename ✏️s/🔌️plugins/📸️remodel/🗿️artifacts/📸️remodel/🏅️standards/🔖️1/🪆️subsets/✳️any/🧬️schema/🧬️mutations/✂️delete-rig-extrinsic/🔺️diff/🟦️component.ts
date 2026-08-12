/** 🔺 delete-rig-extrinsic diff — populates `RemodelDiff.calibration`, or nothing if absent. */
export interface DeleteRigExtrinsicDiff {
  calibration?: { cameras: unknown[]; rig: unknown[] };
}
