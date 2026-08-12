/** ↩️ inverse for `UngroupNode` — always `GroupNodes`. */
export interface UngroupNodeInverseGroupNodes {
  parent: { layer: number; path: number[] };
  indices: number[];
  transform: { translation: { x: number; y: number; z: number }; rotation: { x: number; y: number; z: number; w: number }; scale: { x: number; y: number; z: number } };
}
