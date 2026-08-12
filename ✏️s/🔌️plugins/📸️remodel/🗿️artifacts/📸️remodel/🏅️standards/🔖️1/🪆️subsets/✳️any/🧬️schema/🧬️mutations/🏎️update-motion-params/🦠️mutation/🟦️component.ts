/** ⚙️ update-motion-params mutation payload — full-record replace of one `ReconstructionParams` sub-struct. */
export interface UpdateMotionParams {
  params: { enabled: boolean; maxTracks: number; trackWindowPx: number; minTrackQuality: number; minTrackLengthFrames: number; };
}
