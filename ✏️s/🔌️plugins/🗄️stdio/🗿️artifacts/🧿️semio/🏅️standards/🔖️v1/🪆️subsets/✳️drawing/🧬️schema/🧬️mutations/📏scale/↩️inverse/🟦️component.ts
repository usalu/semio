/** ↩️ inverse for `Scale` — always `Scale` with the captured old scale. */
export interface ScaleInverseScale {
  at: { layer: number; path: number[] };
  newScale: { x: number; y: number; z: number };
}
