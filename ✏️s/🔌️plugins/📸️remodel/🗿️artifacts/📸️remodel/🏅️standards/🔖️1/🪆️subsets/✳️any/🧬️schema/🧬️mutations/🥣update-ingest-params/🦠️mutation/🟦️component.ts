/** ⚙️ update-ingest-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateIngestParams {
  params: { frameSampleStride: number; maxFrames: number; downscaleLongEdgePx: number; minSharpness: number; };
}
