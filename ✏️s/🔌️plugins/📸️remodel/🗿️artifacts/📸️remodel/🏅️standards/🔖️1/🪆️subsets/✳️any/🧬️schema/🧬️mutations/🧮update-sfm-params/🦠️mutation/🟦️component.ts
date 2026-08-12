/** ⚙️ update-sfm-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateSfmParams {
  params: { ransacIterations: number; ransacThresholdPx: number; minTrackLength: number; baMaxIterations: number; robustLoss: "l2" | "huber" | "cauchy"; huberDeltaPx: number; };
}
