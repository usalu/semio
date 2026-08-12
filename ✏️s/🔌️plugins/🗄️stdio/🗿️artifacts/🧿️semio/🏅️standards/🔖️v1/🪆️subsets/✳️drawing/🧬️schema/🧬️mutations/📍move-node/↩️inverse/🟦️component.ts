/** ↩️ inverse for `MoveNode` — always `MoveNode` with the captured old origin. */
export interface MoveNodeInverseMoveNode {
  at: { layer: number; path: number[] };
  newOrigin: { x: number; y: number };
}
