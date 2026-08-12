/** 🔁 replace-qc mutation payload — whole-value swap of `ReconstructionResults.qc`. */
export interface ReplaceQc {
  qc?: { reprojectionRmsPx: number; gcpCheckpointRmse?: number; watertight?: unknown; meanTrackLength: number; registeredFrameRatio: number; denseCoverageRatio: number; warnings: string[] };
}
