/** ⚙️ update-feature-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateFeatureParams {
  params: { detector: "orb" | "akaze" | "harris"; targetCount: number; octaves: number; edgeThreshold: number; };
}
