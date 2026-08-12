/** ⚙️ update-dense-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateDenseParams {
  params: { resolution: "low" | "medium" | "high"; windowRadiusPx: number; minViewConsistency: number; confidenceThreshold: number; maxPoints: number; };
}
