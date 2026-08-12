/** ↩️ inverse for `Rotate` — always `Rotate` with the captured old rotation. */
export interface RotateInverseRotate {
  at: { layer: number; path: number[] };
  newRotation: { x: number; y: number; z: number; w: number };
}
